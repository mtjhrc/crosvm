// Copyright (C) 2019 Alibaba Cloud Computing. All rights reserved.
// SPDX-License-Identifier: Apache-2.0

//! Common data structures for listener and endpoint.

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        pub mod socket;
        mod unix;
    } else if #[cfg(windows)] {
        mod tube;
        pub use tube::TubeEndpoint;
        mod windows;
    }
}

use std::fs::File;
use std::io::IoSlice;
use std::io::IoSliceMut;
use std::mem;
use std::path::Path;

use base::AsRawDescriptor;
use base::RawDescriptor;
use zerocopy::AsBytes;
use zerocopy::FromBytes;

use crate::connection::Req;
use crate::message::MasterReq;
use crate::message::SlaveReq;
use crate::message::*;
use crate::sys::PlatformEndpoint;
use crate::Error;
use crate::Result;
use crate::SystemStream;

/// Listener for accepting connections.
pub trait Listener: Sized {
    /// Accept an incoming connection.
    fn accept(&mut self) -> Result<Option<Endpoint<MasterReq>>>;

    /// Change blocking status on the listener.
    fn set_nonblocking(&self, block: bool) -> Result<()>;
}

// Advance the internal cursor of the slices.
// This is same with a nightly API `IoSlice::advance_slices` but for `&[u8]`.
fn advance_slices(bufs: &mut &mut [&[u8]], mut count: usize) {
    use std::mem::take;

    let mut idx = 0;
    for b in bufs.iter() {
        if count < b.len() {
            break;
        }
        count -= b.len();
        idx += 1;
    }
    *bufs = &mut take(bufs)[idx..];
    if !bufs.is_empty() {
        bufs[0] = &bufs[0][count..];
    }
}

// Advance the internal cursor of the slices.
// This is same with a nightly API `IoSliceMut::advance_slices` but for `&mut [u8]`.
fn advance_slices_mut(bufs: &mut &mut [&mut [u8]], mut count: usize) {
    use std::mem::take;

    let mut idx = 0;
    for b in bufs.iter() {
        if count < b.len() {
            break;
        }
        count -= b.len();
        idx += 1;
    }
    *bufs = &mut take(bufs)[idx..];
    if !bufs.is_empty() {
        let slice = take(&mut bufs[0]);
        let (_, remaining) = slice.split_at_mut(count);
        bufs[0] = remaining;
    }
}

/// A vhost-user connection and related operations.
pub struct Endpoint<R: Req>(pub(crate) PlatformEndpoint<R>);

impl<R: Req> From<SystemStream> for Endpoint<R> {
    fn from(sock: SystemStream) -> Self {
        Self(PlatformEndpoint::from(sock))
    }
}

impl<R: Req> Endpoint<R> {
    /// Create a new stream by connecting to server at `path`.
    pub fn connect<P: AsRef<Path>>(path: P) -> Result<Self> {
        PlatformEndpoint::connect(path).map(Self)
    }

    /// Sends bytes from scatter-gather vectors with optional attached file descriptors.
    ///
    /// # Return:
    /// * - number of bytes sent on success
    pub fn send_iovec(&mut self, iovs: &[IoSlice], fds: Option<&[RawDescriptor]>) -> Result<usize> {
        self.0.send_iovec(iovs, fds)
    }

    /// Reads bytes into the given scatter/gather vectors with optional attached file.
    ///
    /// # Arguements
    /// * `bufs` - A slice of buffers to store received data.
    /// * `allow_fd` - Indicates whether we can receive FDs.
    ///
    /// # Return:
    /// * - (number of bytes received, [received files]) on success.
    /// * - `Error::Disconnect` if the client closed.
    pub fn recv_into_bufs(
        &mut self,
        bufs: &mut [IoSliceMut],
        allow_fd: bool,
    ) -> Result<(usize, Option<Vec<File>>)> {
        self.0.recv_into_bufs(bufs, allow_fd)
    }

    /// Constructs the slave request endpoint for self.
    ///
    /// # Arguments
    /// * `files` - Files from which to create the endpoint
    pub fn create_slave_request_endpoint(
        &mut self,
        files: Option<Vec<File>>,
    ) -> Result<super::Endpoint<SlaveReq>> {
        self.0.create_slave_request_endpoint(files)
    }

    /// Sends all bytes from scatter-gather vectors with optional attached file descriptors. Will
    /// loop until all data has been transfered.
    ///
    /// # TODO
    /// This function takes a slice of `&[u8]` instead of `IoSlice` because the internal
    /// cursor needs to be moved by `advance_slices()`.
    /// Once `IoSlice::advance_slices()` becomes stable, this should be updated.
    /// <https://github.com/rust-lang/rust/issues/62726>.
    pub fn send_iovec_all(
        &self,
        mut iovs: &mut [&[u8]],
        mut fds: Option<&[RawDescriptor]>,
    ) -> Result<()> {
        // Guarantee that `iovs` becomes empty if it doesn't contain any data.
        advance_slices(&mut iovs, 0);

        while !iovs.is_empty() {
            let iovec: Vec<_> = iovs.iter_mut().map(|i| IoSlice::new(i)).collect();
            match self.0.send_iovec(&iovec, fds) {
                Ok(n) => {
                    fds = None;
                    advance_slices(&mut iovs, n);
                }
                Err(e) => match e {
                    Error::SocketRetry(_) => {}
                    _ => return Err(e),
                },
            }
        }
        Ok(())
    }

    /// Sends bytes from a slice with optional attached file descriptors.
    ///
    /// # Return:
    /// * - number of bytes sent on success
    #[cfg(test)]
    pub fn send_slice(&self, data: IoSlice, fds: Option<&[RawDescriptor]>) -> Result<usize> {
        self.0.send_iovec(&[data], fds)
    }

    /// Sends a header-only message with optional attached file descriptors.
    ///
    /// # Return:
    /// * - number of bytes sent on success
    /// * - PartialMessage: received a partial message.
    /// * - backend specific errors
    pub fn send_header(
        &self,
        hdr: &VhostUserMsgHeader<R>,
        fds: Option<&[RawDescriptor]>,
    ) -> Result<()> {
        self.send_iovec_all(&mut [hdr.as_bytes()], fds)
    }

    /// Send a message with header and body. Optional file descriptors may be attached to
    /// the message.
    ///
    /// # Return:
    /// * - number of bytes sent on success
    /// * - OversizedMsg: message size is too big.
    /// * - PartialMessage: received a partial message.
    /// * - backend specific errors
    pub fn send_message<T: Sized + AsBytes>(
        &self,
        hdr: &VhostUserMsgHeader<R>,
        body: &T,
        fds: Option<&[RawDescriptor]>,
    ) -> Result<()> {
        // We send the header and the body separately here. This is necessary on Windows. Otherwise
        // the recv side cannot read the header independently (the transport is message oriented).
        self.send_iovec_all(&mut [hdr.as_bytes()], fds)?;
        self.send_iovec_all(&mut [body.as_bytes()], None)?;
        Ok(())
    }

    /// Send a message with header, body and payload. Optional file descriptors
    /// may also be attached to the message.
    ///
    /// # Return:
    /// * - number of bytes sent on success
    /// * - OversizedMsg: message size is too big.
    /// * - PartialMessage: received a partial message.
    /// * - IncorrectFds: wrong number of attached fds.
    /// * - backend specific errors
    pub fn send_message_with_payload<T: Sized + AsBytes>(
        &self,
        hdr: &VhostUserMsgHeader<R>,
        body: &T,
        payload: &[u8],
        fds: Option<&[RawDescriptor]>,
    ) -> Result<()> {
        if let Some(fd_arr) = fds {
            if fd_arr.len() > MAX_ATTACHED_FD_ENTRIES {
                return Err(Error::IncorrectFds);
            }
        }

        // We send the header and the body separately here. This is necessary on Windows. Otherwise
        // the recv side cannot read the header independently (the transport is message oriented).
        self.send_iovec_all(&mut [hdr.as_bytes()], fds)?;
        self.send_iovec_all(&mut [body.as_bytes(), payload], None)?;

        Ok(())
    }

    /// Reads `len` bytes at most.
    ///
    /// # Return:
    /// * - (number of bytes received, buf) on success
    pub fn recv_data(&self, len: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; len];
        let (data_len, _) = self
            .0
            .recv_into_bufs(&mut [IoSliceMut::new(&mut buf)], false /* allow_fd */)?;
        buf.truncate(data_len);
        Ok(buf)
    }

    /// Reads all bytes into the given scatter/gather vectors with optional attached files. Will
    /// loop until all data has been transfered and errors if EOF is reached before then.
    ///
    /// # Return:
    /// * - received fds on success
    /// * - `Disconnect` - client is closed
    ///
    /// # TODO
    /// This function takes a slice of `&mut [u8]` instead of `IoSliceMut` because the internal
    /// cursor needs to be moved by `advance_slices_mut()`.
    /// Once `IoSliceMut::advance_slices()` becomes stable, this should be updated.
    /// <https://github.com/rust-lang/rust/issues/62726>.
    pub fn recv_into_bufs_all(&self, mut bufs: &mut [&mut [u8]]) -> Result<Option<Vec<File>>> {
        let mut first_read = true;
        let mut rfds = None;

        // Guarantee that `bufs` becomes empty if it doesn't contain any data.
        advance_slices_mut(&mut bufs, 0);

        while !bufs.is_empty() {
            let mut slices: Vec<IoSliceMut> = bufs.iter_mut().map(|b| IoSliceMut::new(b)).collect();
            let res = self.0.recv_into_bufs(&mut slices, true);
            match res {
                Ok((0, _)) => return Err(Error::PartialMessage),
                Ok((n, fds)) => {
                    if first_read {
                        first_read = false;
                        rfds = fds;
                    }
                    advance_slices_mut(&mut bufs, n);
                }
                Err(e) => match e {
                    Error::SocketRetry(_) => {}
                    _ => return Err(e),
                },
            }
        }
        Ok(rfds)
    }

    /// Reads bytes into a new buffer with optional attached files. Received file descriptors are
    /// set close-on-exec and converted to `File`.
    ///
    /// # Return:
    /// * - (number of bytes received, buf, [received files]) on success.
    /// * - backend specific errors
    #[cfg(test)]
    pub fn recv_into_buf(
        &mut self,
        buf_size: usize,
    ) -> Result<(usize, Vec<u8>, Option<Vec<File>>)> {
        let mut buf = vec![0u8; buf_size];
        let mut slices = [IoSliceMut::new(buf.as_mut_slice())];
        let (bytes, files) = self.0.recv_into_bufs(&mut slices, true /* allow_fd */)?;
        Ok((bytes, buf, files))
    }

    /// Receive a header-only message with optional attached files.
    /// Note, only the first MAX_ATTACHED_FD_ENTRIES file descriptors will be
    /// accepted and all other file descriptor will be discard silently.
    ///
    /// # Return:
    /// * - (message header, [received files]) on success.
    /// * - Disconnect: the client closed the connection.
    /// * - PartialMessage: received a partial message.
    /// * - InvalidMessage: received a invalid message.
    /// * - backend specific errors
    pub fn recv_header(&mut self) -> Result<(VhostUserMsgHeader<R>, Option<Vec<File>>)> {
        let mut hdr = VhostUserMsgHeader::default();
        let (bytes, files) = self.0.recv_into_bufs(
            &mut [IoSliceMut::new(hdr.as_bytes_mut())],
            true, /* allow_fd */
        )?;

        if bytes != mem::size_of::<VhostUserMsgHeader<R>>() {
            return Err(Error::PartialMessage);
        } else if !hdr.is_valid() {
            return Err(Error::InvalidMessage);
        }

        Ok((hdr, files))
    }

    /// Receive a message with optional attached file descriptors.
    /// Note, only the first MAX_ATTACHED_FD_ENTRIES file descriptors will be
    /// accepted and all other file descriptor will be discard silently.
    ///
    /// # Return:
    /// * - (message header, message body, [received files]) on success.
    /// * - PartialMessage: received a partial message.
    /// * - InvalidMessage: received a invalid message.
    /// * - backend specific errors
    pub fn recv_body<T: Sized + AsBytes + FromBytes + Default + VhostUserMsgValidator>(
        &self,
    ) -> Result<(VhostUserMsgHeader<R>, T, Option<Vec<File>>)> {
        let mut hdr = VhostUserMsgHeader::default();
        let mut body: T = Default::default();
        let mut slices = [hdr.as_bytes_mut(), body.as_bytes_mut()];
        let files = self.recv_into_bufs_all(&mut slices)?;

        if !hdr.is_valid() || !body.is_valid() {
            return Err(Error::InvalidMessage);
        }

        Ok((hdr, body, files))
    }

    /// Receive a message with header and optional content. Callers need to
    /// pre-allocate a big enough buffer to receive the message body and
    /// optional payload. If there are attached file descriptor associated
    /// with the message, the first MAX_ATTACHED_FD_ENTRIES file descriptors
    /// will be accepted and all other file descriptor will be discard
    /// silently.
    ///
    /// # Return:
    /// * - (message header, [received files]) on success.
    /// * - PartialMessage: received a partial message.
    /// * - InvalidMessage: received a invalid message.
    /// * - backend specific errors
    #[cfg(test)]
    pub fn recv_body_into_buf(
        &self,
        buf: &mut [u8],
    ) -> Result<(VhostUserMsgHeader<R>, Option<Vec<File>>)> {
        let mut hdr = VhostUserMsgHeader::default();
        let mut slices = [hdr.as_bytes_mut(), buf];
        let files = self.recv_into_bufs_all(&mut slices)?;

        if !hdr.is_valid() {
            return Err(Error::InvalidMessage);
        }

        Ok((hdr, files))
    }

    /// Receive a message with optional payload and attached file descriptors.
    /// Note, only the first MAX_ATTACHED_FD_ENTRIES file descriptors will be
    /// accepted and all other file descriptor will be discard silently.
    ///
    /// # Return:
    /// * - (message header, message body, payload, [received files]) on success.
    /// * - PartialMessage: received a partial message.
    /// * - InvalidMessage: received a invalid message.
    /// * - backend specific errors
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::type_complexity))]
    pub fn recv_payload_into_buf<
        T: Sized + AsBytes + FromBytes + Default + VhostUserMsgValidator,
    >(
        &self,
    ) -> Result<(VhostUserMsgHeader<R>, T, Vec<u8>, Option<Vec<File>>)> {
        let mut hdr = VhostUserMsgHeader::default();
        let mut slices = [hdr.as_bytes_mut()];
        let files = self.recv_into_bufs_all(&mut slices)?;

        if !hdr.is_valid() {
            return Err(Error::InvalidMessage);
        }

        let mut body: T = Default::default();
        let payload_size = hdr.get_size() as usize - mem::size_of::<T>();
        let mut buf: Vec<u8> = vec![0; payload_size];
        let mut slices = [body.as_bytes_mut(), buf.as_bytes_mut()];
        let more_files = self.recv_into_bufs_all(&mut slices)?;
        if !body.is_valid() || more_files.is_some() {
            return Err(Error::InvalidMessage);
        }

        Ok((hdr, body, buf, files))
    }
}

impl<R: Req> AsRawDescriptor for Endpoint<R> {
    fn as_raw_descriptor(&self) -> RawDescriptor {
        self.0.as_raw_descriptor()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    cfg_if::cfg_if! {
        if #[cfg(unix)] {
            pub(crate) use super::unix::tests::*;
        } else if #[cfg(windows)] {
            pub(crate) use windows::tests::*;
        }
    }

    #[test]
    fn test_advance_slices() {
        // Test case from https://doc.rust-lang.org/std/io/struct.IoSlice.html#method.advance_slices
        let buf1 = [1; 8];
        let buf2 = [2; 16];
        let buf3 = [3; 8];
        let mut bufs = &mut [&buf1[..], &buf2[..], &buf3[..]][..];
        advance_slices(&mut bufs, 10);
        assert_eq!(bufs[0], [2; 14].as_ref());
        assert_eq!(bufs[1], [3; 8].as_ref());
    }

    #[test]
    fn test_advance_slices_mut() {
        // Test case from https://doc.rust-lang.org/std/io/struct.IoSliceMut.html#method.advance_slices
        let mut buf1 = [1; 8];
        let mut buf2 = [2; 16];
        let mut buf3 = [3; 8];
        let mut bufs = &mut [&mut buf1[..], &mut buf2[..], &mut buf3[..]][..];
        advance_slices_mut(&mut bufs, 10);
        assert_eq!(bufs[0], [2; 14].as_ref());
        assert_eq!(bufs[1], [3; 8].as_ref());
    }
}
