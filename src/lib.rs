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
//! In version 0.1.3 or later, it works well even if the dependency contains only sub-crates such as `futures-core`, `futures-util`, etc.
//!
//! ## Supported traits
//!
//! * [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/future.md)
//! * [`Stream`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.12/futures/stream/trait.Stream.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/stream.md)
//! * [`Sink`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.12/futures/sink/trait.Sink.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/sink.md)
//! * [`AsyncRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.12/futures/io/trait.AsyncRead.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/async_read.md)
//! * [`AsyncWrite`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.12/futures/io/trait.AsyncWrite.html) - [generated code](https://github.com/taiki-e/futures-enum/blob/master/doc/async_write.md)
//!
//! See [this issue](https://github.com/taiki-e/auto_enums/issues/11) for other traits.
//!

#![crate_type = "proc-macro"]
#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/futures-enum/0.1.3")]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unreachable_pub)]

extern crate proc_macro;

use derive_utils::{derive_trait, quick_derive, EnumData as Data, __rt::ident_call_site};
use find_crate::Manifest;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_quote, Ident};

fn crate_name(crate_names: &[&str]) -> (Ident, Option<String>) {
    let f = || (ident_call_site("futures"), None);

    let manifest = match Manifest::new().ok() {
        Some(manifest) => manifest,
        None => return f(),
    };

    manifest
        .find(|name| crate_names.iter().any(|s| *s == name))
        .map(|package| {
            if package.is_original() {
                (
                    ident_call_site(&package.name().replace("_preview", "")),
                    None,
                )
            } else {
                (
                    ident_call_site(package.name()),
                    Some(package.original_name().to_owned()),
                )
            }
        })
        .unwrap_or_else(f)
}

macro_rules! parse {
    ($input:expr) => {
        match syn::parse($input)
            .map_err(derive_utils::Error::from)
            .and_then(|item| Data::from_derive(&item))
        {
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
                waker: &::core::task::Waker
            ) -> ::core::task::Poll<Self::Output>;
        }
    }
}

#[proc_macro_derive(Stream)]
pub fn derive_stream(input: TokenStream) -> TokenStream {
    let (crate_, _) = crate_name(&[
        "futures-preview",
        "futures-util-preview",
        "futures-core-preview",
    ]);

    derive_trait!(
        parse!(input),
        parse_quote!(::#crate_::stream::Stream),
        parse_quote! {
            trait Stream {
                type Item;
                #[inline]
                fn poll_next(
                    self: ::core::pin::Pin<&mut Self>,
                    waker: &::core::task::Waker,
                ) -> ::core::task::Poll<::core::option::Option<Self::Item>>;
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(Sink)]
pub fn derive_sink(input: TokenStream) -> TokenStream {
    let (path, original) = crate_name(&[
        "futures-preview",
        "futures-util-preview",
        "futures-sink-preview",
    ]);

    let path = if path == "futures_sink"
        || original.as_ref().map(|s| s.as_str()) == Some("futures-sink-preview")
    {
        quote!(::#path)
    } else {
        quote!(::#path::sink)
    };

    derive_trait!(
        parse!(input),
        parse_quote!(#path::Sink),
        parse_quote! {
            trait Sink {
                type SinkItem;
                type SinkError;
                #[inline]
                fn poll_ready(
                    self: ::core::pin::Pin<&mut Self>,
                    waker: &::core::task::Waker,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>>;
                #[inline]
                fn start_send(
                    self: ::core::pin::Pin<&mut Self>,
                    item: Self::SinkItem,
                ) -> ::core::result::Result<(), Self::SinkError>;
                #[inline]
                fn poll_flush(
                    self: ::core::pin::Pin<&mut Self>,
                    waker: &::core::task::Waker,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>>;
                #[inline]
                fn poll_close(
                    self: ::core::pin::Pin<&mut Self>,
                    waker: &::core::task::Waker,
                ) -> ::core::task::Poll<::core::result::Result<(), Self::SinkError>>;
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(AsyncRead)]
pub fn derive_async_read(input: TokenStream) -> TokenStream {
    let (path, original) = crate_name(&[
        "futures-preview",
        "futures-util-preview",
        "futures-io-preview",
    ]);

    let path = if path == "futures_io"
        || original.as_ref().map(|s| s.as_str()) == Some("futures-io-preview")
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
                    &mut self,
                    waker: &::core::task::Waker,
                    buf: &mut [u8],
                ) -> ::core::task::Poll<::core::result::Result<usize, #path::Error>>;
                #[inline]
                fn poll_vectored_read(
                    &mut self,
                    waker: &::core::task::Waker,
                    vec: &mut [&mut #path::IoVec],
                ) -> ::core::task::Poll<::core::result::Result<usize, #path::Error>>;
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}

#[proc_macro_derive(AsyncWrite)]
pub fn derive_async_write(input: TokenStream) -> TokenStream {
    let (path, original) = crate_name(&[
        "futures-preview",
        "futures-util-preview",
        "futures-io-preview",
    ]);

    let path = if path == "futures_io"
        || original.as_ref().map(|s| s.as_str()) == Some("futures-io-preview")
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
                    &mut self,
                    waker: &::core::task::Waker,
                    buf: &[u8],
                ) -> ::core::task::Poll<::core::result::Result<usize, #path::Error>>;
                #[inline]
                fn poll_vectored_write(
                    &mut self,
                    waker: &::core::task::Waker,
                    vec: &[&#path::IoVec],
                ) -> ::core::task::Poll<::core::result::Result<usize, #path::Error>>;
                #[inline]
                fn poll_flush(
                    &mut self,
                    waker: &::core::task::Waker,
                ) -> ::core::task::Poll<::core::result::Result<(), #path::Error>>;
                #[inline]
                fn poll_close(
                    &mut self,
                    waker: &::core::task::Waker,
                ) -> ::core::task::Poll<::core::result::Result<(), #path::Error>>;
            }
        },
    )
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}
