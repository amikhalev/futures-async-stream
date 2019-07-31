# futures-async-stream

<!-- [![Build Status][azure-badge]][azure-url] -->
[![Crates.io][crates-version-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![License][crates-license-badge]][crates-url]
![Minimum supported Rust version][rustc-badge]

<!-- [azure-badge]: -->
<!-- [azure-url]: -->
[crates-version-badge]: https://img.shields.io/crates/v/futures-async-stream.svg
[crates-license-badge]: https://img.shields.io/crates/l/futures-async-stream.svg
[crates-badge]: https://img.shields.io/crates/v/futures-async-stream.svg
[crates-url]: https://crates.io/crates/futures-async-stream/
[docs-badge]: https://docs.rs/futures-async-stream/badge.svg
[docs-url]: https://docs.rs/futures-async-stream/
[rustc-badge]: https://img.shields.io/badge/rustc-nightly-lightgray.svg

Async stream API experiment that may be introduced as a language feature in the future.

This crate provides useful features for streams, using unstable `async_await` and `generators`.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
futures-async-stream = "0.1.0-alpha.1"
futures-preview = "0.3.0-alpha.17"
```

The current futures-async-stream requires Rust nightly 2019-05-09 or later.

## \#\[for_await\]

Processes streams using a for loop.

This is a reimplement of [futures-await]'s `#[async]` for loops for futures 0.3 and is an experimental implementation of [the idea listed as the next step of async/await](https://github.com/rust-lang/rfcs/blob/master/text/2394-async_await.md#for-await-and-processing-streams).

```rust
#![feature(async_await, stmt_expr_attributes, proc_macro_hygiene)]
use futures::prelude::*;
use futures_async_stream::for_await;

async fn collect(stream: impl Stream<Item = i32>) -> Vec<i32> {
    let mut vec = Vec::new();
    #[for_await]
    for value in stream {
        vec.push(value);
    }
    vec
}
```

`value` has the `Item` type of the stream passed in. Note that async for loops can only be used inside of `async` functions, closures, blocks, `#[async_stream]` functions and `async_stream_block!` macros.

## \#\[async_stream\]

Creates streams via generators.

This is a reimplement of [futures-await]'s `#[async_stream]` for futures 0.3 and is an experimental implementation of [the idea listed as the next step of async/await](https://github.com/rust-lang/rfcs/blob/master/text/2394-async_await.md#generators-and-streams).

```rust
#![feature(async_await, generators)]
use futures::prelude::*;
use futures_async_stream::async_stream;

// Returns a stream of i32
#[async_stream(item = i32)]
async fn foo(stream: impl Stream<Item = String>) {
    // `for_await` is built into `async_stream`. If you use `for_await` only in `async_stream`, there is no need to import `for_await`.
    #[for_await]
    for x in stream {
        yield x.parse().unwrap();
    }
}
```

`#[async_stream]` must have an item type specified via `item = some::Path` and the values output from the stream must be yielded via the `yield` expression.

<!--
### async_stream_block!

TODO
-->

<!--
## List of features that may be added in the future as an extension of this feature:

  * `async_try_stream` (https://github.com/rust-lang-nursery/futures-rs/pull/1548#discussion_r287558350)
  * `async_sink` (https://github.com/rust-lang-nursery/futures-rs/pull/1548#issuecomment-486205382)
  * Support `.await` in macro (https://github.com/rust-lang-nursery/futures-rs/pull/1548#discussion_r285341883)
  * Parallel version of `for_await` (https://github.com/rustasync/runtime/pull/25)
-->

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
