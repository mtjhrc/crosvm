/* automatically generated by tools/bindgen-all-the-things */

#![allow(clippy::missing_safety_doc)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
pub const IORING_FILE_INDEX_ALLOC: i32 = -1;
pub const IORING_SETUP_IOPOLL: u32 = 1;
pub const IORING_SETUP_SQPOLL: u32 = 2;
pub const IORING_SETUP_SQ_AFF: u32 = 4;
pub const IORING_SETUP_CQSIZE: u32 = 8;
pub const IORING_SETUP_CLAMP: u32 = 16;
pub const IORING_SETUP_ATTACH_WQ: u32 = 32;
pub const IORING_SETUP_R_DISABLED: u32 = 64;
pub const IORING_SETUP_SUBMIT_ALL: u32 = 128;
pub const IORING_SETUP_COOP_TASKRUN: u32 = 256;
pub const IORING_SETUP_TASKRUN_FLAG: u32 = 512;
pub const IORING_SETUP_SQE128: u32 = 1024;
pub const IORING_SETUP_CQE32: u32 = 2048;
pub const IORING_SETUP_SINGLE_ISSUER: u32 = 4096;
pub const IORING_SETUP_DEFER_TASKRUN: u32 = 8192;
pub const IORING_URING_CMD_FIXED: u32 = 1;
pub const IORING_FSYNC_DATASYNC: u32 = 1;
pub const IORING_TIMEOUT_ABS: u32 = 1;
pub const IORING_TIMEOUT_UPDATE: u32 = 2;
pub const IORING_TIMEOUT_BOOTTIME: u32 = 4;
pub const IORING_TIMEOUT_REALTIME: u32 = 8;
pub const IORING_LINK_TIMEOUT_UPDATE: u32 = 16;
pub const IORING_TIMEOUT_ETIME_SUCCESS: u32 = 32;
pub const IORING_TIMEOUT_CLOCK_MASK: u32 = 12;
pub const IORING_TIMEOUT_UPDATE_MASK: u32 = 18;
pub const IORING_POLL_ADD_MULTI: u32 = 1;
pub const IORING_POLL_UPDATE_EVENTS: u32 = 2;
pub const IORING_POLL_UPDATE_USER_DATA: u32 = 4;
pub const IORING_POLL_ADD_LEVEL: u32 = 8;
pub const IORING_ASYNC_CANCEL_ALL: u32 = 1;
pub const IORING_ASYNC_CANCEL_FD: u32 = 2;
pub const IORING_ASYNC_CANCEL_ANY: u32 = 4;
pub const IORING_ASYNC_CANCEL_FD_FIXED: u32 = 8;
pub const IORING_RECVSEND_POLL_FIRST: u32 = 1;
pub const IORING_RECV_MULTISHOT: u32 = 2;
pub const IORING_RECVSEND_FIXED_BUF: u32 = 4;
pub const IORING_SEND_ZC_REPORT_USAGE: u32 = 8;
pub const IORING_NOTIF_USAGE_ZC_COPIED: u32 = 2147483648;
pub const IORING_ACCEPT_MULTISHOT: u32 = 1;
pub const IORING_MSG_RING_CQE_SKIP: u32 = 1;
pub const IORING_CQE_F_BUFFER: u32 = 1;
pub const IORING_CQE_F_MORE: u32 = 2;
pub const IORING_CQE_F_SOCK_NONEMPTY: u32 = 4;
pub const IORING_CQE_F_NOTIF: u32 = 8;
pub const IORING_OFF_SQ_RING: u32 = 0;
pub const IORING_OFF_CQ_RING: u32 = 134217728;
pub const IORING_OFF_SQES: u32 = 268435456;
pub const IORING_SQ_NEED_WAKEUP: u32 = 1;
pub const IORING_SQ_CQ_OVERFLOW: u32 = 2;
pub const IORING_SQ_TASKRUN: u32 = 4;
pub const IORING_CQ_EVENTFD_DISABLED: u32 = 1;
pub const IORING_ENTER_GETEVENTS: u32 = 1;
pub const IORING_ENTER_SQ_WAKEUP: u32 = 2;
pub const IORING_ENTER_SQ_WAIT: u32 = 4;
pub const IORING_ENTER_EXT_ARG: u32 = 8;
pub const IORING_ENTER_REGISTERED_RING: u32 = 16;
pub const IORING_FEAT_SINGLE_MMAP: u32 = 1;
pub const IORING_FEAT_NODROP: u32 = 2;
pub const IORING_FEAT_SUBMIT_STABLE: u32 = 4;
pub const IORING_FEAT_RW_CUR_POS: u32 = 8;
pub const IORING_FEAT_CUR_PERSONALITY: u32 = 16;
pub const IORING_FEAT_FAST_POLL: u32 = 32;
pub const IORING_FEAT_POLL_32BITS: u32 = 64;
pub const IORING_FEAT_SQPOLL_NONFIXED: u32 = 128;
pub const IORING_FEAT_EXT_ARG: u32 = 256;
pub const IORING_FEAT_NATIVE_WORKERS: u32 = 512;
pub const IORING_FEAT_RSRC_TAGS: u32 = 1024;
pub const IORING_FEAT_CQE_SKIP: u32 = 2048;
pub const IORING_FEAT_LINKED_FILE: u32 = 4096;
pub const IORING_RSRC_REGISTER_SPARSE: u32 = 1;
pub const IORING_REGISTER_FILES_SKIP: i32 = -2;
pub const IO_URING_OP_SUPPORTED: u32 = 1;
pub type __kernel_time64_t = ::std::os::raw::c_longlong;
pub type __kernel_rwf_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct __kernel_timespec {
    pub tv_sec: __kernel_time64_t,
    pub tv_nsec: ::std::os::raw::c_longlong,
}
#[repr(C)]
pub struct io_uring_sqe {
    pub opcode: u8,
    pub flags: u8,
    pub ioprio: u16,
    pub fd: i32,
    pub __bindgen_anon_1: io_uring_sqe__bindgen_ty_1,
    pub __bindgen_anon_2: io_uring_sqe__bindgen_ty_2,
    pub len: u32,
    pub __bindgen_anon_3: io_uring_sqe__bindgen_ty_3,
    pub user_data: u64,
    pub __bindgen_anon_4: io_uring_sqe__bindgen_ty_4,
    pub personality: u16,
    pub __bindgen_anon_5: io_uring_sqe__bindgen_ty_5,
    pub __bindgen_anon_6: io_uring_sqe__bindgen_ty_6,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_1 {
    pub off: u64,
    pub addr2: u64,
    pub __bindgen_anon_1: io_uring_sqe__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_sqe__bindgen_ty_1__bindgen_ty_1 {
    pub cmd_op: u32,
    pub __pad1: u32,
}
impl Default for io_uring_sqe__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_2 {
    pub addr: u64,
    pub splice_off_in: u64,
}
impl Default for io_uring_sqe__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_3 {
    pub rw_flags: __kernel_rwf_t,
    pub fsync_flags: u32,
    pub poll_events: u16,
    pub poll32_events: u32,
    pub sync_range_flags: u32,
    pub msg_flags: u32,
    pub timeout_flags: u32,
    pub accept_flags: u32,
    pub cancel_flags: u32,
    pub open_flags: u32,
    pub statx_flags: u32,
    pub fadvise_advice: u32,
    pub splice_flags: u32,
    pub rename_flags: u32,
    pub unlink_flags: u32,
    pub hardlink_flags: u32,
    pub xattr_flags: u32,
    pub msg_ring_flags: u32,
    pub uring_cmd_flags: u32,
}
impl Default for io_uring_sqe__bindgen_ty_3 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_4 {
    pub buf_index: u16,
    pub buf_group: u16,
}
impl Default for io_uring_sqe__bindgen_ty_4 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_5 {
    pub splice_fd_in: i32,
    pub file_index: u32,
    pub __bindgen_anon_1: io_uring_sqe__bindgen_ty_5__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_sqe__bindgen_ty_5__bindgen_ty_1 {
    pub addr_len: u16,
    pub __pad3: [u16; 1usize],
}
impl Default for io_uring_sqe__bindgen_ty_5 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct io_uring_sqe__bindgen_ty_6 {
    pub __bindgen_anon_1: __BindgenUnionField<io_uring_sqe__bindgen_ty_6__bindgen_ty_1>,
    pub cmd: __BindgenUnionField<[u8; 0usize]>,
    pub bindgen_union_field: [u64; 2usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_sqe__bindgen_ty_6__bindgen_ty_1 {
    pub addr3: u64,
    pub __pad2: [u64; 1usize],
}
impl Default for io_uring_sqe__bindgen_ty_6 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for io_uring_sqe {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const io_uring_op_IORING_OP_NOP: io_uring_op = 0;
pub const io_uring_op_IORING_OP_READV: io_uring_op = 1;
pub const io_uring_op_IORING_OP_WRITEV: io_uring_op = 2;
pub const io_uring_op_IORING_OP_FSYNC: io_uring_op = 3;
pub const io_uring_op_IORING_OP_READ_FIXED: io_uring_op = 4;
pub const io_uring_op_IORING_OP_WRITE_FIXED: io_uring_op = 5;
pub const io_uring_op_IORING_OP_POLL_ADD: io_uring_op = 6;
pub const io_uring_op_IORING_OP_POLL_REMOVE: io_uring_op = 7;
pub const io_uring_op_IORING_OP_SYNC_FILE_RANGE: io_uring_op = 8;
pub const io_uring_op_IORING_OP_SENDMSG: io_uring_op = 9;
pub const io_uring_op_IORING_OP_RECVMSG: io_uring_op = 10;
pub const io_uring_op_IORING_OP_TIMEOUT: io_uring_op = 11;
pub const io_uring_op_IORING_OP_TIMEOUT_REMOVE: io_uring_op = 12;
pub const io_uring_op_IORING_OP_ACCEPT: io_uring_op = 13;
pub const io_uring_op_IORING_OP_ASYNC_CANCEL: io_uring_op = 14;
pub const io_uring_op_IORING_OP_LINK_TIMEOUT: io_uring_op = 15;
pub const io_uring_op_IORING_OP_CONNECT: io_uring_op = 16;
pub const io_uring_op_IORING_OP_FALLOCATE: io_uring_op = 17;
pub const io_uring_op_IORING_OP_OPENAT: io_uring_op = 18;
pub const io_uring_op_IORING_OP_CLOSE: io_uring_op = 19;
pub const io_uring_op_IORING_OP_FILES_UPDATE: io_uring_op = 20;
pub const io_uring_op_IORING_OP_STATX: io_uring_op = 21;
pub const io_uring_op_IORING_OP_READ: io_uring_op = 22;
pub const io_uring_op_IORING_OP_WRITE: io_uring_op = 23;
pub const io_uring_op_IORING_OP_FADVISE: io_uring_op = 24;
pub const io_uring_op_IORING_OP_MADVISE: io_uring_op = 25;
pub const io_uring_op_IORING_OP_SEND: io_uring_op = 26;
pub const io_uring_op_IORING_OP_RECV: io_uring_op = 27;
pub const io_uring_op_IORING_OP_OPENAT2: io_uring_op = 28;
pub const io_uring_op_IORING_OP_EPOLL_CTL: io_uring_op = 29;
pub const io_uring_op_IORING_OP_SPLICE: io_uring_op = 30;
pub const io_uring_op_IORING_OP_PROVIDE_BUFFERS: io_uring_op = 31;
pub const io_uring_op_IORING_OP_REMOVE_BUFFERS: io_uring_op = 32;
pub const io_uring_op_IORING_OP_TEE: io_uring_op = 33;
pub const io_uring_op_IORING_OP_SHUTDOWN: io_uring_op = 34;
pub const io_uring_op_IORING_OP_RENAMEAT: io_uring_op = 35;
pub const io_uring_op_IORING_OP_UNLINKAT: io_uring_op = 36;
pub const io_uring_op_IORING_OP_MKDIRAT: io_uring_op = 37;
pub const io_uring_op_IORING_OP_SYMLINKAT: io_uring_op = 38;
pub const io_uring_op_IORING_OP_LINKAT: io_uring_op = 39;
pub const io_uring_op_IORING_OP_MSG_RING: io_uring_op = 40;
pub const io_uring_op_IORING_OP_FSETXATTR: io_uring_op = 41;
pub const io_uring_op_IORING_OP_SETXATTR: io_uring_op = 42;
pub const io_uring_op_IORING_OP_FGETXATTR: io_uring_op = 43;
pub const io_uring_op_IORING_OP_GETXATTR: io_uring_op = 44;
pub const io_uring_op_IORING_OP_SOCKET: io_uring_op = 45;
pub const io_uring_op_IORING_OP_URING_CMD: io_uring_op = 46;
pub const io_uring_op_IORING_OP_SEND_ZC: io_uring_op = 47;
pub const io_uring_op_IORING_OP_SENDMSG_ZC: io_uring_op = 48;
pub const io_uring_op_IORING_OP_LAST: io_uring_op = 49;
pub type io_uring_op = ::std::os::raw::c_uint;
pub const IORING_MSG_DATA: _bindgen_ty_2 = 0;
pub const IORING_MSG_SEND_FD: _bindgen_ty_2 = 1;
pub type _bindgen_ty_2 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Default)]
pub struct io_uring_cqe {
    pub user_data: u64,
    pub res: i32,
    pub flags: u32,
    pub big_cqe: __IncompleteArrayField<u64>,
}
pub const IORING_CQE_BUFFER_SHIFT: _bindgen_ty_3 = 16;
pub type _bindgen_ty_3 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_sqring_offsets {
    pub head: u32,
    pub tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub flags: u32,
    pub dropped: u32,
    pub array: u32,
    pub resv1: u32,
    pub resv2: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_cqring_offsets {
    pub head: u32,
    pub tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub overflow: u32,
    pub cqes: u32,
    pub flags: u32,
    pub resv1: u32,
    pub resv2: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_params {
    pub sq_entries: u32,
    pub cq_entries: u32,
    pub flags: u32,
    pub sq_thread_cpu: u32,
    pub sq_thread_idle: u32,
    pub features: u32,
    pub wq_fd: u32,
    pub resv: [u32; 3usize],
    pub sq_off: io_sqring_offsets,
    pub cq_off: io_cqring_offsets,
}
pub const IORING_REGISTER_BUFFERS: _bindgen_ty_4 = 0;
pub const IORING_UNREGISTER_BUFFERS: _bindgen_ty_4 = 1;
pub const IORING_REGISTER_FILES: _bindgen_ty_4 = 2;
pub const IORING_UNREGISTER_FILES: _bindgen_ty_4 = 3;
pub const IORING_REGISTER_EVENTFD: _bindgen_ty_4 = 4;
pub const IORING_UNREGISTER_EVENTFD: _bindgen_ty_4 = 5;
pub const IORING_REGISTER_FILES_UPDATE: _bindgen_ty_4 = 6;
pub const IORING_REGISTER_EVENTFD_ASYNC: _bindgen_ty_4 = 7;
pub const IORING_REGISTER_PROBE: _bindgen_ty_4 = 8;
pub const IORING_REGISTER_PERSONALITY: _bindgen_ty_4 = 9;
pub const IORING_UNREGISTER_PERSONALITY: _bindgen_ty_4 = 10;
pub const IORING_REGISTER_RESTRICTIONS: _bindgen_ty_4 = 11;
pub const IORING_REGISTER_ENABLE_RINGS: _bindgen_ty_4 = 12;
pub const IORING_REGISTER_FILES2: _bindgen_ty_4 = 13;
pub const IORING_REGISTER_FILES_UPDATE2: _bindgen_ty_4 = 14;
pub const IORING_REGISTER_BUFFERS2: _bindgen_ty_4 = 15;
pub const IORING_REGISTER_BUFFERS_UPDATE: _bindgen_ty_4 = 16;
pub const IORING_REGISTER_IOWQ_AFF: _bindgen_ty_4 = 17;
pub const IORING_UNREGISTER_IOWQ_AFF: _bindgen_ty_4 = 18;
pub const IORING_REGISTER_IOWQ_MAX_WORKERS: _bindgen_ty_4 = 19;
pub const IORING_REGISTER_RING_FDS: _bindgen_ty_4 = 20;
pub const IORING_UNREGISTER_RING_FDS: _bindgen_ty_4 = 21;
pub const IORING_REGISTER_PBUF_RING: _bindgen_ty_4 = 22;
pub const IORING_UNREGISTER_PBUF_RING: _bindgen_ty_4 = 23;
pub const IORING_REGISTER_SYNC_CANCEL: _bindgen_ty_4 = 24;
pub const IORING_REGISTER_FILE_ALLOC_RANGE: _bindgen_ty_4 = 25;
pub const IORING_REGISTER_LAST: _bindgen_ty_4 = 26;
pub type _bindgen_ty_4 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_files_update {
    pub offset: u32,
    pub resv: u32,
    pub fds: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_rsrc_register {
    pub nr: u32,
    pub flags: u32,
    pub resv2: u64,
    pub data: u64,
    pub tags: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_rsrc_update {
    pub offset: u32,
    pub resv: u32,
    pub data: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_rsrc_update2 {
    pub offset: u32,
    pub resv: u32,
    pub data: u64,
    pub tags: u64,
    pub nr: u32,
    pub resv2: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_notification_slot {
    pub tag: u64,
    pub resv: [u64; 3usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_notification_register {
    pub nr_slots: u32,
    pub resv: u32,
    pub resv2: u64,
    pub data: u64,
    pub resv3: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_probe_op {
    pub op: u8,
    pub resv: u8,
    pub flags: u16,
    pub resv2: u32,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct io_uring_probe {
    pub last_op: u8,
    pub ops_len: u8,
    pub resv: u16,
    pub resv2: [u32; 3usize],
    pub ops: __IncompleteArrayField<io_uring_probe_op>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_restriction {
    pub opcode: u16,
    pub __bindgen_anon_1: io_uring_restriction__bindgen_ty_1,
    pub resv: u8,
    pub resv2: [u32; 3usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_restriction__bindgen_ty_1 {
    pub register_op: u8,
    pub sqe_op: u8,
    pub sqe_flags: u8,
}
impl Default for io_uring_restriction__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for io_uring_restriction {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_buf {
    pub addr: u64,
    pub len: u32,
    pub bid: u16,
    pub resv: u16,
}
#[repr(C)]
pub struct io_uring_buf_ring {
    pub __bindgen_anon_1: io_uring_buf_ring__bindgen_ty_1,
}
#[repr(C)]
pub struct io_uring_buf_ring__bindgen_ty_1 {
    pub __bindgen_anon_1: __BindgenUnionField<io_uring_buf_ring__bindgen_ty_1__bindgen_ty_1>,
    pub bufs: __BindgenUnionField<[io_uring_buf; 0usize]>,
    pub bindgen_union_field: [u64; 2usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_buf_ring__bindgen_ty_1__bindgen_ty_1 {
    pub resv1: u64,
    pub resv2: u32,
    pub resv3: u16,
    pub tail: u16,
}
impl Default for io_uring_buf_ring__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for io_uring_buf_ring {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_buf_reg {
    pub ring_addr: u64,
    pub ring_entries: u32,
    pub bgid: u16,
    pub pad: u16,
    pub resv: [u64; 3usize],
}
pub const IORING_RESTRICTION_REGISTER_OP: _bindgen_ty_6 = 0;
pub const IORING_RESTRICTION_SQE_OP: _bindgen_ty_6 = 1;
pub const IORING_RESTRICTION_SQE_FLAGS_ALLOWED: _bindgen_ty_6 = 2;
pub const IORING_RESTRICTION_SQE_FLAGS_REQUIRED: _bindgen_ty_6 = 3;
pub const IORING_RESTRICTION_LAST: _bindgen_ty_6 = 4;
pub type _bindgen_ty_6 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_getevents_arg {
    pub sigmask: u64,
    pub sigmask_sz: u32,
    pub pad: u32,
    pub ts: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_sync_cancel_reg {
    pub addr: u64,
    pub fd: i32,
    pub flags: u32,
    pub timeout: __kernel_timespec,
    pub pad: [u64; 4usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_file_index_range {
    pub off: u32,
    pub len: u32,
    pub resv: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_recvmsg_out {
    pub namelen: u32,
    pub controllen: u32,
    pub payloadlen: u32,
    pub flags: u32,
}
