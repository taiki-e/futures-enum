//! \#\[derive(Future, Stream, Sink, AsyncRead, AsyncWrite, AsyncSeek, AsyncBufRead)\] for enums.
//!
//! ## Examples
//!
//! ```rust
//! use futures_enum::*;
//! use std::future::Future;
//!
//! #[derive(Future, Stream, Sink, AsyncRead, AsyncWrite, AsyncSeek, AsyncBufRead)]
//! enum Either<A, B> {
//!     A(A),
//!     B(B),
//! }
//!
//! fn foo(x: i32) -> impl Future<Output = i32> {
//!     if x < 0 { Either::A(async { 1 }) } else { Either::B(async move { x }) }
//! }
//! ```
//!
//! futures-enum works well even if the dependency contains only sub-crates
//! such as `futures-core`, `futures-io`, `futures-sink`, etc.
//!
//! See [auto_enums](https://github.com/taiki-e/auto_enums) for how to automate patterns like this.
//!
//! ## Supported traits
//!
//! * [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/future.md)
//! * [`Stream`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/stream/trait.Stream.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/stream.md)
//! * [`Sink`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/sink/trait.Sink.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/sink.md)
//! * [`AsyncRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/io/trait.AsyncRead.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/async_read.md)
//! * [`AsyncWrite`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/io/trait.AsyncWrite.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/async_write.md)
//! * [`AsyncSeek`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/io/trait.AsyncSeek.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/async_seek.md)
//! * [`AsyncBufRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/io/trait.AsyncBufRead.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/async_buf_read.md)

#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/futures-enum/0.1.11")]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms, single_use_lifetimes), allow(dead_code))
))]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![warn(clippy::all, clippy::pedantic)]

extern crate proc_macro;

use derive_utils::{derive_trait, quick_derive, EnumData as Data};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, Ident};

fn default_crate_name() -> (Ident, Option<String>) {
    (format_ident!("futures"), None)
}

#[cfg(feature = "renamed")]
fn crate_name(crate_names: &[&str]) -> (Ident, Option<String>) {
    use find_crate::Manifest;

    let manifest = match Manifest::new().ok() {
        Some(manifest) => manifest,
        None => return default_crate_name(),
    };

    manifest.find(|name| crate_names.iter().any(|s| *s == name)).map_or_else(
        default_crate_name,
        |package| {
            if package.is_original() {
                (format_ident!("{}", package.name.replace("_preview", "")), None)
            } else {
                (format_ident!("{}", &package.name), Some(package.original_name().to_owned()))
            }
        },
    )
}

#[cfg(not(feature = "renamed"))]
fn crate_name(_: &[&str]) -> (Ident, Option<String>) {
    default_crate_name()
}

macro_rules! parse {
    ($input:expr) => {
        match syn::parse($input).and_then(|item: syn::DeriveInput| Data::new(&item)) {
            Ok(data) => data,
            Err(e) => return e.to_compile_error().into(),
        }
    };
}

#[proc_macro_derive(Future)]
pub fn derive_future(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        (::core::future::Future),
        trait Future {
            type Output;
            #[inline]
            fn poll(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<Self::Output>;
        }
    }
}

#[proc_macro_derive(Stream)]
pub fn derive_stream(input: TokenStream) -> TokenStream {
    let (crate_, _) = crate_name(&["futures-preview", "futures-core-preview"]);

    derive_trait!(
        parse!(input),
        parse_quote!(::#crate_::stream::Stream),
        parse_quote! {
            trait Stream {
                type Item;
                #[inline]
                fn poll_next(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::option::Option<Self::Item>>;
                #[inline]
                fn size_hint(&self) -> (usize, ::core::option::Option<usize>);
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(Sink)]
pub fn derive_sink(input: TokenStream) -> TokenStream {
    let (crate_, original) = crate_name(&["futures-preview", "futures-sink-preview"]);

    let path = if original.as_ref().map_or(false, |s| s == "futures-sink-preview") {
        quote!(::#crate_)
    } else {
        quote!(::#crate_::sink)
    };

    derive_trait!(
        parse!(input),
        parse_quote!(#path::Sink),
        parse_quote! {
            trait Sink<__Item> {
                type Error;
                #[inline]
                fn poll_ready(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>>;
                #[inline]
                fn start_send(
                    self: ::core::pin::Pin<&mut Self>,
                    item: __Item,
                ) -> ::core::result::Result<(), Self::Error>;
                #[inline]
                fn poll_flush(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>>;
                #[inline]
                fn poll_close(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::Error>>;
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(AsyncRead)]
pub fn derive_async_read(input: TokenStream) -> TokenStream {
    let (crate_, original) = crate_name(&["futures-preview", "futures-io-preview"]);

    let path = if original.as_ref().map_or(false, |s| s == "futures-io-preview") {
        quote!(::#crate_)
    } else {
        quote!(::#crate_::io)
    };

    derive_trait!(
        parse!(input),
        parse_quote!(#path::AsyncRead),
        parse_quote! {
            trait AsyncRead {
                #[inline]
                fn poll_read(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    buf: &mut [u8],
                ) -> ::core::task::Poll<::std::io::Result<usize>>;
                #[inline]
                fn poll_read_vectored(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    bufs: &mut [::std::io::IoSliceMut<'_>],
                ) -> ::core::task::Poll<::std::io::Result<usize>>;
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(AsyncWrite)]
pub fn derive_async_write(input: TokenStream) -> TokenStream {
    let (crate_, original) = crate_name(&["futures-preview", "futures-io-preview"]);

    let path = if original.as_ref().map_or(false, |s| s == "futures-io-preview") {
        quote!(::#crate_)
    } else {
        quote!(::#crate_::io)
    };

    derive_trait!(
        parse!(input),
        parse_quote!(#path::AsyncWrite),
        parse_quote! {
            trait AsyncWrite {
                #[inline]
                fn poll_write(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    buf: &[u8],
                ) -> ::core::task::Poll<::std::io::Result<usize>>;
                #[inline]
                fn poll_write_vectored(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    bufs: &[::std::io::IoSlice<'_>],
                ) -> ::core::task::Poll<::std::io::Result<usize>>;
                #[inline]
                fn poll_flush(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::std::io::Result<()>>;
                #[inline]
                fn poll_close(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::std::io::Result<()>>;
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(AsyncSeek)]
pub fn derive_async_seek(input: TokenStream) -> TokenStream {
    let (crate_, original) = crate_name(&["futures-preview", "futures-io-preview"]);

    let path = if original.as_ref().map_or(false, |s| s == "futures-io-preview") {
        quote!(::#crate_)
    } else {
        quote!(::#crate_::io)
    };

    derive_trait!(
        parse!(input),
        parse_quote!(#path::AsyncSeek),
        parse_quote! {
            trait AsyncSeek {
                #[inline]
                fn poll_seek(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                    pos: ::std::io::SeekFrom,
                ) -> ::core::task::Poll<::std::io::Result<u64>>;
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(AsyncBufRead)]
pub fn derive_async_buf_read(input: TokenStream) -> TokenStream {
    let (crate_, original) = crate_name(&["futures-preview", "futures-io-preview"]);

    let path = if original.as_ref().map_or(false, |s| s == "futures-io-preview") {
        quote!(::#crate_)
    } else {
        quote!(::#crate_::io)
    };

    derive_trait!(
        parse!(input),
        parse_quote!(#path::AsyncBufRead),
        parse_quote! {
            trait AsyncBufRead {
                #[inline]
                fn poll_fill_buf<'__a>(
                    self: ::core::pin::Pin<&'__a mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::std::io::Result<&'__a [u8]>>;
                #[inline]
                fn consume(self: ::core::pin::Pin<&mut Self>, amt: usize);
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}
