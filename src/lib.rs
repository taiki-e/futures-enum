//! \#\[derive(Future, Stream, Sink, AsyncRead, AsyncWrite, AsyncSeek, AsyncBufRead)\] for enums.
//!
//! ## Examples
//!
//! ```rust
//! use futures::future::{self, Future};
//! use futures_enum::*;
//!
//! #[derive(Future, Stream, Sink, AsyncRead, AsyncWrite, AsyncSeek, AsyncBufRead)]
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
//! futures-enum works well even if the dependency contains only sub-crates such as `futures-core`, `futures-util`, etc.
//!
//! ## Supported traits
//!
//! * [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/future.md)
//! * [`Stream`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/stream/trait.Stream.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/stream.md)
//! * [`Sink`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/sink/trait.Sink.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/sink.md)
//! * [`AsyncRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/io/trait.AsyncRead.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/async_read.md)
//! * [`AsyncWrite`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/io/trait.AsyncWrite.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/async_write.md)
//! * [`AsyncSeek`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/io/trait.AsyncSeek.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/async_seek.md)
//! * [`AsyncBufRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/io/trait.AsyncBufRead.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/async_buf_read.md)
//!

#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/futures-enum/0.1.8")]
#![doc(test(attr(deny(warnings), allow(dead_code, unused_assignments, unused_variables))))]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms, unreachable_pub)]
#![warn(single_use_lifetimes)]
#![warn(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]

extern crate proc_macro;

use derive_utils::{derive_trait, quick_derive, EnumData as Data};
use find_crate::Manifest;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_quote, Ident};

fn ident<S: AsRef<str>>(s: S) -> Ident {
    Ident::new(s.as_ref(), Span::call_site())
}

fn crate_name(crate_names: &[&str]) -> (Ident, Option<String>) {
    let f = || (ident("futures"), None);

    let manifest = match Manifest::new().ok() {
        Some(manifest) => manifest,
        None => return f(),
    };

    manifest.find(|name| crate_names.iter().any(|s| *s == name)).map_or_else(f, |package| {
        if package.is_original() {
            (ident(&package.name().replace("_preview", "")), None)
        } else {
            (ident(package.name()), Some(package.original_name().to_owned()))
        }
    })
}

macro_rules! parse {
    ($input:expr) => {
        match syn::parse($input).and_then(|item| Data::from_derive(&item)) {
            Ok(data) => data,
            Err(err) => return TokenStream::from(err.to_compile_error()),
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
            fn poll(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<Self::Output>;
        }
    }
}

#[proc_macro_derive(Stream)]
pub fn derive_stream(input: TokenStream) -> TokenStream {
    let (crate_, _) =
        crate_name(&["futures-preview", "futures-util-preview", "futures-core-preview"]);

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
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(Sink)]
pub fn derive_sink(input: TokenStream) -> TokenStream {
    let (path, original) =
        crate_name(&["futures-preview", "futures-util-preview", "futures-sink-preview"]);

    let path = if path == "futures_sink"
        || original.as_ref().map(String::as_str) == Some("futures-sink-preview")
    {
        quote!(::#path)
    } else {
        quote!(::#path::sink)
    };

    derive_trait!(
        parse!(input),
        parse_quote!(#path::Sink),
        parse_quote! {
            trait Sink<Item> {
                type SinkError;
                #[inline]
                fn poll_ready(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>>;
                #[inline]
                fn start_send(
                    self: ::core::pin::Pin<&mut Self>,
                    item: Item,
                ) -> ::core::result::Result<(), Self::SinkError>;
                #[inline]
                fn poll_flush(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>>;
                #[inline]
                fn poll_close(
                    self: ::core::pin::Pin<&mut Self>,
                    cx: &mut ::core::task::Context<'_>,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>>;
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(AsyncRead)]
pub fn derive_async_read(input: TokenStream) -> TokenStream {
    let (path, original) =
        crate_name(&["futures-preview", "futures-util-preview", "futures-io-preview"]);

    let path = if path == "futures_io"
        || original.as_ref().map(String::as_str) == Some("futures-io-preview")
    {
        quote!(::#path)
    } else {
        quote!(::#path::io)
    };

    derive_trait!(
        parse!(input),
        parse_quote!(#path::AsyncRead),
        parse_quote! {
            trait AsyncRead {
                #[inline]
                unsafe fn initializer(&self) -> #path::Initializer;
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
    let (path, original) =
        crate_name(&["futures-preview", "futures-util-preview", "futures-io-preview"]);

    let path = if path == "futures_io"
        || original.as_ref().map(String::as_str) == Some("futures-io-preview")
    {
        quote!(::#path)
    } else {
        quote!(::#path::io)
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
    let (path, original) =
        crate_name(&["futures-preview", "futures-util-preview", "futures-io-preview"]);

    let path = if path == "futures_io"
        || original.as_ref().map(String::as_str) == Some("futures-io-preview")
    {
        quote!(::#path)
    } else {
        quote!(::#path::io)
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
    let (path, original) =
        crate_name(&["futures-preview", "futures-util-preview", "futures-io-preview"]);

    let path = if path == "futures_io"
        || original.as_ref().map(String::as_str) == Some("futures-io-preview")
    {
        quote!(::#path)
    } else {
        quote!(::#path::io)
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
