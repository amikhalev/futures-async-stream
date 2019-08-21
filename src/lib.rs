//! Async stream API experiment that may be introduced as a language feature in the future.
//!
//! This crate provides useful features for streams, using `async_await` and unstable `generators`.
//!
//! ## \#\[for_await\]
//!
//! Processes streams using a for loop.
//!
//! This is a reimplement of [futures-await]'s `#[async]` for loops for futures 0.3 and is an experimental implementation of [the idea listed as the next step of async/await](https://github.com/rust-lang/rfcs/blob/master/text/2394-async_await.md#for-await-and-processing-streams).
//!
//! ```rust
//! #![feature(stmt_expr_attributes, proc_macro_hygiene)]
//! use futures::stream::Stream;
//! use futures_async_stream::for_await;
//!
//! async fn collect(stream: impl Stream<Item = i32>) -> Vec<i32> {
//!     let mut vec = Vec::new();
//!     #[for_await]
//!     for value in stream {
//!         vec.push(value);
//!     }
//!     vec
//! }
//! ```
//!
//! `value` has the `Item` type of the stream passed in. Note that async for loops can only be used inside of `async` functions, closures, blocks, `#[async_stream]` functions and `async_stream_block!` macros.
//!
//! ## \#\[async_stream\]
//!
//! Creates streams via generators.
//!
//! This is a reimplement of [futures-await]'s `#[async_stream]` for futures 0.3 and is an experimental implementation of [the idea listed as the next step of async/await](https://github.com/rust-lang/rfcs/blob/master/text/2394-async_await.md#generators-and-streams).
//!
//! ```rust
//! #![feature(generators)]
//! use futures::stream::Stream;
//! use futures_async_stream::async_stream;
//!
//! // Returns a stream of i32
//! #[async_stream(item = i32)]
//! async fn foo(stream: impl Stream<Item = String>) {
//!     // `for_await` is built into `async_stream`. If you use `for_await` only in `async_stream`, there is no need to import `for_await`.
//!     #[for_await]
//!     for x in stream {
//!         yield x.parse().unwrap();
//!     }
//! }
//! ```
//!
//! `#[async_stream]` must have an item type specified via `item = some::Path` and the values output from the stream must be yielded via the `yield` expression.
//!
//! ## async_stream_block!
//!
//! You can create a stream directly as an expression using an `async_stream_block!` macro:
//!
//! ```rust
//! #![feature(generators, proc_macro_hygiene)]
//! use futures::stream::Stream;
//! use futures_async_stream::async_stream_block;
//!
//! fn foo() -> impl Stream<Item = i32> {
//!     async_stream_block! {
//!         for i in 0..10 {
//!             yield i;
//!         }
//!     }
//! }
//! ```
//!
//! ## Using async stream functions in traits
//!
//! You can use async stream functions in traits by passing `boxed` or `boxed_local` as an argument.
//!
//! ```rust
//! #![feature(generators)]
//! use futures_async_stream::async_stream;
//!
//! trait Foo {
//!     #[async_stream(boxed, item = u32)]
//!     async fn method(&mut self);
//! }
//!
//! struct Bar(u32);
//!
//! impl Foo for Bar {
//!     #[async_stream(boxed, item = u32)]
//!     async fn method(&mut self) {
//!         while self.0 < u32::max_value() {
//!             self.0 += 1;
//!             yield self.0;
//!         }
//!     }
//! }
//! ```
//!
//! A async stream function that received a `boxed` argument is converted to a function that returns `Pin<Box<dyn Stream<Item = item> + Send + 'lifetime>>`.
//! If you passed `boxed_local` instead of `boxed`, async stream function returns a non-threadsafe stream (`Pin<Box<dyn Stream<Item = item> + 'lifetime>>`).
//!
//! ```rust
//! #![feature(generators)]
//! use futures::stream::Stream;
//! use futures_async_stream::async_stream;
//! use std::pin::Pin;
//!
//! // The trait itself can be defined without unstable features.
//! trait Foo {
//!     fn method(&mut self) -> Pin<Box<dyn Stream<Item = u32> + Send + '_>>;
//! }
//!
//! struct Bar(u32);
//!
//! impl Foo for Bar {
//!     #[async_stream(boxed, item = u32)]
//!     async fn method(&mut self) {
//!         while self.0 < u32::max_value() {
//!             self.0 += 1;
//!             yield self.0;
//!         }
//!     }
//! }
//! ```
//!
//! ## How to write the equivalent code without this API?
//!
//! ### \#\[for_await\]
//!
//! You can write this by combining `while let` loop, `.await`, `pin_mut` macro, and `StreamExt::next()` method:
//!
//! ```rust
//! use futures::{
//!     pin_mut,
//!     stream::{Stream, StreamExt},
//! };
//!
//! async fn collect(stream: impl Stream<Item = i32>) -> Vec<i32> {
//!     let mut vec = Vec::new();
//!     pin_mut!(stream);
//!     while let Some(value) = stream.next().await {
//!         vec.push(value);
//!     }
//!     vec
//! }
//! ```
//!
//! ### \#\[async_stream\]
//!
//! You can write this by manually implementing the combinator:
//!
//! ```rust
//! use futures::{
//!     stream::Stream,
//!     ready,
//!     task::{Context, Poll},
//! };
//! use pin_project::pin_project;
//! use std::pin::Pin;
//!
//! fn foo<S>(stream: S) -> impl Stream<Item = i32>
//! where
//!     S: Stream<Item = String>,
//! {
//!     Foo { stream }
//! }
//!
//! #[pin_project]
//! struct Foo<S> {
//!     #[pin]
//!     stream: S,
//! }
//!
//! impl<S> Stream for Foo<S>
//! where
//!     S: Stream<Item = String>,
//! {
//!     type Item = i32;
//!
//!     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//!         let this = self.project();
//!         if let Some(x) = ready!(this.stream.poll_next(cx)) {
//!             Poll::Ready(Some(x.parse().unwrap()))
//!         } else {
//!             Poll::Ready(None)
//!         }
//!     }
//! }
//! ```
//!
//! [futures-await]: https://github.com/alexcrichton/futures-await

#![doc(html_root_url = "https://docs.rs/futures-async-stream/0.1.0-alpha.3")]
#![doc(test(attr(deny(warnings), allow(dead_code, unused_assignments, unused_variables))))]
#![warn(rust_2018_idioms, unreachable_pub, single_use_lifetimes)]
#![warn(clippy::all, clippy::pedantic)]
#![feature(gen_future, generator_trait)]

extern crate alloc;

/// Processes streams using a for loop.
pub use futures_async_stream_macro::for_await;

/// Creates streams via generators.
pub use futures_async_stream_macro::async_stream;

/// Creates streams via generators.
pub use futures_async_stream_macro::async_stream_block;

#[doc(hidden)]
pub mod stream;

#[doc(hidden)]
pub mod reexport {
    #[doc(hidden)]
    pub use core::{option, pin, result, task};

    #[doc(hidden)]
    pub use alloc::boxed;
}
