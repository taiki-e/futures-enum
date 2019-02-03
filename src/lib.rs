//! \#\[derive(Future, Stream, Sink, AsyncRead, AsyncWrite)\] for enums.
//!
//! ## Examples
//!
//! ```rust
//! # #![feature(futures_api)]
//! use futures::future::{self, Future};
//! use futures_enum::*;
//!
//! #[derive(Future, Stream, Sink, AsyncRead, AsyncWrite)]
//! enum Either<A, B> {
//!     A(A),
//!     B(B),
//! }
//!
//! fn foo(x: i32) -> impl Future<Output = i32> {
//!     if x < 0 {
//!         Either::A(future::lazy(|_| 1))
//!     } else {
//!         Either::B(future::ready(x))
//!     }
//! }
//! ```
//!
//! See [auto_enums](https://github.com/taiki-e/auto_enums) for how to automate patterns like this.
//!
//! ## Supported traits
//!
//! * [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html)
//! * [`Stream`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.12/futures/stream/trait.Stream.html)
//! * [`Sink`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.12/futures/sink/trait.Sink.html)
//! * [`AsyncRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.12/futures/io/trait.AsyncRead.html)
//! * [`AsyncWrite`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.12/futures/io/trait.AsyncWrite.html)
//!
//! See [auto_enums#11](https://github.com/taiki-e/auto_enums/issues/11) for other traits.
//!

#![crate_type = "proc-macro"]
#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/futures-enum/0.1.1")]
#![deny(rust_2018_idioms)]

extern crate proc_macro;

use derive_utils::quick_derive;
use proc_macro::TokenStream;

#[proc_macro_derive(Future)]
pub fn derive_future(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        (::core::future::Future),
        trait Future {
            type Output;
            fn poll(
                self: ::core::pin::Pin<&mut Self>,
                lw: &::core::task::LocalWaker
            ) -> ::core::task::Poll<Self::Output>;
        }
    }
}

#[proc_macro_derive(Stream)]
pub fn derive_stream(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        (::futures::stream::Stream),
        trait Stream {
            type Item;
            #[inline]
            fn poll_next(
                self: ::core::pin::Pin<&mut Self>,
                lw: &::core::task::LocalWaker,
            ) -> ::core::task::Poll<::core::option::Option<Self::Item>>;
        }
    }
}

#[proc_macro_derive(Sink)]
pub fn derive_sink(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        (::futures::sink::Sink),
        trait Sink {
            type SinkItem;
            type SinkError;
            #[inline]
            fn poll_ready(
                self: ::core::pin::Pin<&mut Self>,
                lw: &::core::task::LocalWaker,
            ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>>;
            #[inline]
            fn start_send(
                self: ::core::pin::Pin<&mut Self>,
                item: Self::SinkItem,
            ) -> ::core::result::Result<(), Self::SinkError>;
            #[inline]
            fn poll_flush(
                self: ::core::pin::Pin<&mut Self>,
                lw: &::core::task::LocalWaker,
            ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>>;
            #[inline]
            fn poll_close(
                self: ::core::pin::Pin<&mut Self>,
                lw: &::core::task::LocalWaker,
            ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>>;
        }
    }
}

#[proc_macro_derive(AsyncRead)]
pub fn derive_async_read(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        (::futures::io::AsyncRead),
        trait AsyncRead {
            #[inline]
            unsafe fn initializer(&self) -> ::futures::io::Initializer;
            #[inline]
            fn poll_read(
                &mut self,
                lw: &::core::task::LocalWaker,
                buf: &mut [u8],
            ) -> ::core::task::Poll<::core::result::Result<usize, ::futures::io::Error>>;
            #[inline]
            fn poll_vectored_read(
                &mut self,
                lw: &::core::task::LocalWaker,
                vec: &mut [&mut ::futures::io::IoVec],
            ) -> ::core::task::Poll<::core::result::Result<usize, ::futures::io::Error>>;
        }
    }
}

#[proc_macro_derive(AsyncWrite)]
pub fn derive_async_write(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        (::futures::io::AsyncWrite),
        trait AsyncWrite {
            #[inline]
            fn poll_write(
                &mut self,
                lw: &::core::task::LocalWaker,
                buf: &[u8],
            ) -> ::core::task::Poll<::core::result::Result<usize, ::futures::io::Error>>;
            #[inline]
            fn poll_vectored_write(
                &mut self,
                lw: &::core::task::LocalWaker,
                vec: &[&::futures::io::IoVec],
            ) -> ::core::task::Poll<::core::result::Result<usize, ::futures::io::Error>>;
            #[inline]
            fn poll_flush(
                &mut self,
                lw: &::core::task::LocalWaker,
            ) -> ::core::task::Poll<::core::result::Result<(), ::futures::io::Error>>;
            #[inline]
            fn poll_close(
                &mut self,
                lw: &::core::task::LocalWaker,
            ) -> ::core::task::Poll<::core::result::Result<(), ::futures::io::Error>>;
        }
    }
}
