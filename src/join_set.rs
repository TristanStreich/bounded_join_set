use std::{
    fmt,
    future::Future,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
    task::{Context, Poll},
};

use tokio::{sync::Semaphore, task::JoinSet as TokioJoinSet};

use crate::tokio_exports::{AbortHandle, Handle, JoinError, LocalSet};

/// Same as [`tokio::task::JoinSet`] except the number of actively polled futures is limited to a set concurrency.
///
/// Does not support [`spawn_blocking`](https://docs.rs/tokio/1.32.0/tokio/task/struct.JoinSet.html#method.spawn_blocking).
///
/// For any undocumented methods see [`tokio::task::JoinSet`].
///
/// [`tokio::task::JoinSet`]: https://docs.rs/tokio/1.32.0/tokio/task/struct.JoinSet.html
pub struct JoinSet<T> {
    num_inactive_tasks: Arc<AtomicUsize>,
    active_semaphore: Arc<Semaphore>,
    inner_join_set: TokioJoinSet<T>,
    concurrency: usize,
}

impl<T> JoinSet<T> {
    /// Creates a new JoinSet with a set concurrency
    ///
    /// Panics if concurrency is larger than [`JoinSet::MAX_CONCURRENCY`]
    pub fn new(concurrency: usize) -> Self {
        Self {
            num_inactive_tasks: Arc::new(AtomicUsize::new(0)),
            inner_join_set: TokioJoinSet::new(),
            active_semaphore: Arc::new(Semaphore::new(concurrency)),
            concurrency,
        }
    }

    /// Returns number of active tasks, queued tasks, and completed tasks
    pub fn len(&self) -> usize {
        self.inner_join_set.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// number of futures actively being polled
    pub fn num_active(&self) -> usize {
        self.concurrency - self.active_semaphore.available_permits()
    }

    /// number of tasks which have been pushed to the join set and are waiting to begin work
    pub fn num_queued(&self) -> usize {
        self.num_inactive_tasks.load(Ordering::Acquire)
    }

    /// number of tasks that have already been completed but have not been removed from the join set with [`join_next`](#method.join_next)
    pub fn num_completed(&self) -> usize {
        self.len() - self.num_active() - self.num_queued()
    }

    pub const MAX_CONCURRENCY: usize = Semaphore::MAX_PERMITS;
}

impl<T: 'static> JoinSet<T> {
    fn wrap_task<F>(&self, task: F) -> impl Future<Output = T> + 'static
    where
        F: Future<Output = T> + 'static,
    {
        self.num_inactive_tasks.fetch_add(1, Ordering::Release);

        let task_semaphore = self.active_semaphore.clone();
        let task_inactive_count = self.num_inactive_tasks.clone();

        async move {
            // SAFETY: error here means the semaphore is closed which is currently not logically possible
            let _permit = task_semaphore.acquire_owned().await.unwrap();

            task_inactive_count.fetch_sub(1, Ordering::Release);

            let output = task.await;

            output
        }
    }

    pub fn spawn<F>(&mut self, task: F) -> AbortHandle
    where
        F: Future<Output = T> + Send + 'static,
        T: Send,
    {
        self.inner_join_set.spawn(self.wrap_task(task))
    }

    pub fn spawn_on<F>(&mut self, task: F, handle: &Handle) -> AbortHandle
    where
        F: Future<Output = T> + Send + 'static,
        T: Send,
    {
        self.inner_join_set.spawn_on(self.wrap_task(task), handle)
    }

    pub fn spawn_local<F>(&mut self, task: F) -> AbortHandle
    where
        F: Future<Output = T> + 'static,
    {
        self.inner_join_set.spawn_local(self.wrap_task(task))
    }

    pub fn spawn_local_on<F>(&mut self, task: F, local_set: &LocalSet) -> AbortHandle
    where
        F: Future<Output = T> + 'static,
    {
        self.inner_join_set
            .spawn_local_on(self.wrap_task(task), local_set)
    }

    pub async fn join_next(&mut self) -> Option<Result<T, JoinError>> {
        self.inner_join_set.join_next().await
    }

    pub fn try_join_next(&mut self) -> Option<Result<T, JoinError>> {
        self.inner_join_set.try_join_next()
    }

    pub async fn shutdown(&mut self) {
        self.inner_join_set.shutdown().await;
    }

    pub fn abort_all(&mut self) {
        self.inner_join_set.abort_all();
    }

    pub fn detach_all(&mut self) {
        self.inner_join_set.detach_all();
    }

    pub fn poll_join_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<T, JoinError>>> {
        self.inner_join_set.poll_join_next(cx)
    }
}

impl<T> Default for JoinSet<T> {
    /// Default concurrency is 8
    fn default() -> Self {
        Self::new(8)
    }
}

impl<T> Drop for JoinSet<T> {
    fn drop(&mut self) {}
}

impl<T> fmt::Debug for JoinSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinSet")
            .field("len", &self.len())
            .field("active_tasks", &self.num_active())
            .field("queued_tasks", &self.num_queued())
            .field("num_completed", &self.num_completed())
            .field("concurrency", &self.concurrency)
            .finish()
    }
}
