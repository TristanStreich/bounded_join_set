# bounded_join_set

A Concurrency-Limited JoinSet for Tokio.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/bounded_join_set.svg
[crates-url]: https://crates.io/crates/bounded_join_set
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/TristanStreich/bounded_join_set/blob/main/LICENSE
[actions-badge]: https://github.com/TristanStreich/bounded_join_set/workflows/Tests/badge.svg
[actions-url]: https://github.com/TristanStreich/bounded_join_set/actions?query=branch%3Amain++

<!-- cargo-rdme start -->

The `bounded_join_set` crate provides a simple wrapper around Tokio's `JoinSet` with a configurable concurrency limit.
It exposes the same API as the native `JoinSet`, but ensures that no matter how many tasks you add to the set,
only a predetermined number (based on the concurrency limit) will be polled concurrently.

### Why use `bounded_join_set`?

In scenarios where you want to spawn multiple concurrent tasks using Tokio, but want to avoid overwhelming the system
with too many simultaneous tasks, `bounded_join_set` comes to the rescue. It ensures that you won't exhaust
system resources, while still benefiting from concurrency.

### How to use

Add `bounded_join_set` to your `Cargo.toml` dependencies:

```toml
[dependencies]
bounded_join_set = "0.3.0"
```

Here's a basic usage example:

```rust
use bounded_join_set::JoinSet;

#[tokio::main]
async fn main() {
    let concurrency_limit = 5;
    let mut join_set = JoinSet::new(concurrency_limit);

    for _ in 0..10 {
        join_set.spawn(async {
            // Your concurrent task here.
        });
    }

    while join_set.join_next().await.is_some() {}
}
```

### Features & Benefits

- **Simple API**: If you're familiar with Tokio's `JoinSet`, you already know how to use `bounded_join_set`.
- **Resource Management**: Prevents system overloads by controlling the number of concurrently polled tasks.
- **Flexible**: Set your own concurrency limit according to your system's capabilities and requirements.

### Limitations

While `bounded_join_set` aims to offer an enhanced concurrency control over Tokio's native `JoinSet`,
there are certain limitations to be aware of:

- **No `spawn_blocking` Support**: As of the current version, the crate does not support the `spawn_blocking` method
  provided by Tokio. If your application relies heavily on CPU-bound operations that would benefit from the `spawn_blocking` feature,
  you'll need to manage that separately from this crate. We are looking into extending support for this in future releases,
  but there's no definite timeline yet.

<!-- cargo-rdme end -->

We're always keen on improving and expanding the capabilities of `bounded_join_set`. If you encounter any issues or 
have suggestions, please feel free to open an issue on our GitHub repository.


### Contribution & Support

Contributions, feature requests, and feedback are always welcome. Feel free to open an issue or a pull request on the GitHub repository.

__License:__ [MIT](LICENSE)

