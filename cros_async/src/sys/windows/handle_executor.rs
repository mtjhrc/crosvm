// Copyright 2020 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::future::Future;
use std::io;
use std::pin::Pin;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Weak;

use async_task::Runnable;
use async_task::Task;
use futures::task::Context;
use futures::task::Poll;
use pin_utils::pin_mut;
use sync::Condvar;
use sync::Mutex;
use thiserror::Error as ThisError;

use crate::queue::RunnableQueue;
use crate::waker::new_waker;
use crate::waker::WeakWake;
use crate::DetachedTasks;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Failed to get future from executor run.")]
    FailedToReadFutureFromWakerChannel(mpsc::RecvError),
}

impl From<Error> for io::Error {
    fn from(e: Error) -> Self {
        use Error::*;
        match e {
            FailedToReadFutureFromWakerChannel(e) => io::Error::new(io::ErrorKind::Other, e),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct HandleExecutorTaskHandle<R> {
    task: Task<R>,
    raw: Weak<RawExecutor>,
}

impl<R: Send + 'static> HandleExecutorTaskHandle<R> {
    pub fn detach(self) {
        if let Some(raw) = self.raw.upgrade() {
            raw.detached_tasks.lock().push(self.task);
        }
    }
}

impl<R: 'static> Future for HandleExecutorTaskHandle<R> {
    type Output = R;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Self::Output> {
        Pin::new(&mut self.task).poll(cx)
    }
}

#[derive(Clone)]
pub struct HandleExecutor {
    raw: Arc<RawExecutor>,
}

impl HandleExecutor {
    pub fn new() -> Self {
        Self {
            raw: Arc::new(RawExecutor::new()),
        }
    }

    pub fn spawn<F>(&self, f: F) -> HandleExecutorTaskHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.raw.spawn(f)
    }

    pub fn spawn_local<F>(&self, f: F) -> HandleExecutorTaskHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        self.raw.spawn_local(f)
    }

    pub fn run_until<F: Future>(&self, f: F) -> Result<F::Output> {
        let waker = new_waker(Arc::downgrade(&self.raw));
        let mut cx = Context::from_waker(&waker);
        self.raw.run(&mut cx, f)
    }
}

struct RawExecutor {
    queue: RunnableQueue,
    woken: Mutex<bool>,
    wakeup: Condvar,
    detached_tasks: Mutex<DetachedTasks>,
}

impl RawExecutor {
    fn new() -> Self {
        Self {
            queue: RunnableQueue::new(),
            woken: Mutex::new(false),
            wakeup: Condvar::new(),
            detached_tasks: Mutex::new(DetachedTasks::new()),
        }
    }

    fn make_schedule_fn(self: &Arc<Self>) -> impl Fn(Runnable) {
        let raw = Arc::downgrade(self);
        move |runnable| {
            if let Some(r) = raw.upgrade() {
                r.queue.push_back(runnable);
                r.wake();
            }
        }
    }

    fn spawn<F>(self: &Arc<Self>, f: F) -> HandleExecutorTaskHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let (runnable, task) = async_task::spawn(f, self.make_schedule_fn());
        runnable.schedule();
        HandleExecutorTaskHandle {
            task,
            raw: Arc::downgrade(self),
        }
    }

    fn spawn_local<F>(self: &Arc<Self>, f: F) -> HandleExecutorTaskHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        let (runnable, task) = async_task::spawn_local(f, self.make_schedule_fn());
        runnable.schedule();
        HandleExecutorTaskHandle {
            task,
            raw: Arc::downgrade(self),
        }
    }

    fn run<F: Future>(&self, cx: &mut Context, done: F) -> Result<F::Output> {
        pin_mut!(done);

        loop {
            for runnable in self.queue.iter() {
                runnable.run();
            }
            if let Ok(mut tasks) = self.detached_tasks.try_lock() {
                tasks.poll(cx);
            }
            if let Poll::Ready(val) = done.as_mut().poll(cx) {
                return Ok(val);
            }

            self.wait()
        }
    }

    fn wait(&self) {
        let mut woken = self.woken.lock();
        while !*woken {
            woken = self.wakeup.wait(woken);
        }
        *woken = false;
    }

    fn wake(self: &Arc<Self>) {
        *self.woken.lock() = true;
        self.wakeup.notify_one();
    }
}

impl WeakWake for RawExecutor {
    fn wake_by_ref(weak_self: &Weak<Self>) {
        if let Some(arc_self) = weak_self.upgrade() {
            RawExecutor::wake(&arc_self);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const FUT_MSG: i32 = 5;
    use crate::BlockingPool;
    use futures::channel::mpsc as fut_mpsc;
    use futures::SinkExt;
    use futures::StreamExt;
    use std::rc::Rc;

    #[test]
    fn run_future() {
        let (send, recv) = mpsc::channel();
        async fn this_test(send: mpsc::Sender<i32>) {
            send.send(FUT_MSG).unwrap();
        }

        let ex = HandleExecutor::new();
        ex.run_until(this_test(send)).unwrap();
        assert_eq!(recv.recv().unwrap(), FUT_MSG);
    }

    #[test]
    fn spawn_future() {
        let (send, recv) = fut_mpsc::channel(1);
        let (send_done_signal, receive_done_signal) = mpsc::channel();

        async fn message_sender(mut send: fut_mpsc::Sender<i32>) {
            send.send(FUT_MSG).await.unwrap();
        }

        async fn message_receiver(mut recv: fut_mpsc::Receiver<i32>, done: mpsc::Sender<bool>) {
            assert_eq!(recv.next().await.unwrap(), FUT_MSG);
            done.send(true).unwrap();
        }

        let ex = HandleExecutor::new();
        ex.spawn(message_sender(send)).detach();
        ex.run_until(message_receiver(recv, send_done_signal))
            .unwrap();
        assert_eq!(receive_done_signal.recv().unwrap(), true);
    }

    // Dropping a task that owns a BlockingPool shouldn't leak the pool.
    #[test]
    fn drop_detached_blocking_pool() {
        struct Cleanup(BlockingPool);

        impl Drop for Cleanup {
            fn drop(&mut self) {
                // Make sure we shutdown cleanly (BlockingPool::drop just prints a warning).
                self.0
                    .shutdown(Some(
                        std::time::Instant::now() + std::time::Duration::from_secs(1),
                    ))
                    .unwrap();
            }
        }

        let rc = Rc::new(std::cell::Cell::new(0));
        {
            let ex = HandleExecutor::new();
            let rc_clone = rc.clone();
            ex.spawn_local(async move {
                rc_clone.set(1);
                let pool = Cleanup(BlockingPool::new(1, std::time::Duration::new(60, 0)));
                let (send, recv) = std::sync::mpsc::sync_channel::<()>(0);
                // Spawn a blocking task.
                let blocking_task = pool.0.spawn(move || {
                    // Rendezvous.
                    assert_eq!(recv.recv(), Ok(()));
                    // Wait for drop.
                    assert_eq!(recv.recv(), Err(std::sync::mpsc::RecvError));
                });
                // Make sure it has actually started (using a "rendezvous channel" send).
                //
                // Without this step, we'll have a race where we can shutdown the blocking pool
                // before the worker thread pops off the task.
                send.send(()).unwrap();
                // Wait for it to finish
                blocking_task.await;
                rc_clone.set(2);
            })
            .detach();
            ex.run_until(async {}).unwrap();
            // `ex` is dropped here. If everything is working as expected, it should drop all of
            // its tasks, including `send` and `pool` (in that order, which is important). `pool`'s
            // `Drop` impl will try to join all the worker threads, which should work because send
            // half of the channel closed.
        }
        assert_eq!(rc.get(), 1);
        Rc::try_unwrap(rc).expect("Rc had too many refs");
    }
}
