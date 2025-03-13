//! The `bounded_join_set` crate provides a simple wrapper around Tokio's `JoinSet` with a configurable concurrency limit.
//! It exposes the same API as the native `JoinSet`, but ensures that no matter how many tasks you add to the set,
//! only a predetermined number (based on the concurrency limit) will be polled concurrently.
//!
//! ## Why use `bounded_join_set`?
//!
//! In scenarios where you want to spawn multiple concurrent tasks using Tokio, but want to avoid overwhelming the system
//! with too many simultaneous tasks, `bounded_join_set` comes to the rescue. It ensures that you won't exhaust
//! system resources, while still benefiting from concurrency.
//!
//! ## How to use
//!
//! Add `bounded_join_set` to your `Cargo.toml` dependencies:
//!
//! ```toml
//! [dependencies]
//! bounded_join_set = "0.3.0"
//! ```
//!
//! Here's a basic usage example:
//!
//! ```rust
//! use bounded_join_set::JoinSet;
//!
//! #[tokio::main]
//! async fn main() {
//!     let concurrency_limit = 5;
//!     let mut join_set = JoinSet::new(concurrency_limit);
//!
//!     for _ in 0..10 {
//!         join_set.spawn(async {
//!             // Your concurrent task here.
//!         });
//!     }
//!
//!     while join_set.join_next().await.is_some() {}
//! }
//! ```
//!
//! ## Features & Benefits
//!
//! - **Simple API**: If you're familiar with Tokio's `JoinSet`, you already know how to use `bounded_join_set`.
//! - **Resource Management**: Prevents system overloads by controlling the number of concurrently polled tasks.
//! - **Flexible**: Set your own concurrency limit according to your system's capabilities and requirements.
//!
//! ## Limitations
//!
//! While `bounded_join_set` aims to offer an enhanced concurrency control over Tokio's native `JoinSet`,
//! there are certain limitations to be aware of:
//!
//! - **No `spawn_blocking` Support**: As of the current version, the crate does not support the `spawn_blocking` method
//!   provided by Tokio. If your application relies heavily on CPU-bound operations that would benefit from the `spawn_blocking` feature,
//!   you'll need to manage that separately from this crate. We are looking into extending support for this in future releases,
//!   but there's no definite timeline yet.

mod join_set;
pub use join_set::*;

/// Re-export tokio types that are exposed in the api
pub mod tokio_exports {
    pub use tokio::{
        runtime::Handle,
        task::{AbortHandle, Id, JoinError, LocalSet},
    };
}
