// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
<!-- tidy:crate-doc:start -->
\#\[derive(Future, Stream, Sink, AsyncRead, AsyncWrite, AsyncSeek, AsyncBufRead)\] for enums.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
futures-enum = "0.1.16"
futures = "0.3"
```

*Compiler support: requires rustc 1.56+*

## Examples

```rust
use std::future::Future;

use futures_enum::*;

#[derive(Future, Stream, Sink, AsyncRead, AsyncWrite, AsyncSeek, AsyncBufRead)]
enum Either<A, B> {
    A(A),
    B(B),
}

fn foo(x: i32) -> impl Future<Output = i32> {
    if x < 0 {
        Either::A(async { 1 })
    } else {
        Either::B(async move { x })
    }
}
```

futures-enum works well even if the dependency contains only sub-crates such
as `futures-core`, `futures-io`, `futures-sink`, etc.

See [auto_enums] crate for how to automate patterns like this.

## Supported traits

- [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html) - [example](https://github.com/taiki-e/futures-enum/blob/HEAD/tests/expand/future.rs) | [generated code](https://github.com/taiki-e/futures-enum/blob/HEAD/tests/expand/future.expanded.rs)
- [`Stream`](https://docs.rs/futures/0.3/futures/stream/trait.Stream.html) - [example](https://github.com/taiki-e/futures-enum/blob/HEAD/tests/expand/stream.rs) | [generated code](https://github.com/taiki-e/futures-enum/blob/HEAD/tests/expand/stream.expanded.rs)
- [`Sink`](https://docs.rs/futures/0.3/futures/sink/trait.Sink.html) - [example](https://github.com/taiki-e/futures-enum/blob/HEAD/tests/expand/sink.rs) | [generated code](https://github.com/taiki-e/futures-enum/blob/HEAD/tests/expand/sink.expanded.rs)
- [`AsyncRead`](https://docs.rs/futures/0.3/futures/io/trait.AsyncRead.html) - [example](https://github.com/taiki-e/futures-enum/blob/HEAD/tests/expand/async_read.rs) | [generated code](https://github.com/taiki-e/futures-enum/blob/HEAD/tests/expand/async_read.expanded.rs)
- [`AsyncWrite`](https://docs.rs/futures/0.3/futures/io/trait.AsyncWrite.html) - [example](https://github.com/taiki-e/futures-enum/blob/HEAD/tests/expand/async_write.rs) | [generated code](https://github.com/taiki-e/futures-enum/blob/HEAD/tests/expand/async_write.expanded.rs)
- [`AsyncSeek`](https://docs.rs/futures/0.3/futures/io/trait.AsyncSeek.html) - [example](https://github.com/taiki-e/futures-enum/blob/HEAD/tests/expand/async_seek.rs) | [generated code](https://github.com/taiki-e/futures-enum/blob/HEAD/tests/expand/async_seek.expanded.rs)
- [`AsyncBufRead`](https://docs.rs/futures/0.3/futures/io/trait.AsyncBufRead.html) - [example](https://github.com/taiki-e/futures-enum/blob/HEAD/tests/expand/async_buf_read.rs) | [generated code](https://github.com/taiki-e/futures-enum/blob/HEAD/tests/expand/async_buf_read.expanded.rs)

## Related Projects

- [auto_enums]: A library for to allow multiple return types by automatically generated enum.
- [derive_utils]: A procedural macro helper for easily writing [derives macros][proc-macro-derive] for enums.
- [io-enum]: \#\[derive(Read, Write, Seek, BufRead)\] for enums.
- [iter-enum]: \#\[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, Extend)\] for enums.

[auto_enums]: https://github.com/taiki-e/auto_enums
[derive_utils]: https://github.com/taiki-e/derive_utils
[io-enum]: https://github.com/taiki-e/io-enum
[iter-enum]: https://github.com/taiki-e/iter-enum
[proc-macro-derive]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros

<!-- tidy:crate-doc:end -->
*/

#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables)
    )
))]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes, unreachable_pub, clippy::pedantic)]
#![allow(clippy::doc_markdown)]

use derive_utils::{derive_trait, quick_derive};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, Ident};

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

    manifest
        .find2(|name, version| {
            if name == "futures" {
                let mut pieces = version.split('.');
                (|| pieces.next()?.parse().ok())() == Some(3)
            } else {
                crate_names.iter().any(|s| *s == name)
            }
        })
        .map_or_else(default_crate_name, |package| {
            if package.is_original() {
                (format_ident!("{}", package.name), None)
            } else {
                (format_ident!("{}", &package.name), Some(package.original_name().to_owned()))
            }
        })
}

#[cfg(not(feature = "renamed"))]
fn crate_name(_: &[&str]) -> (Ident, Option<String>) {
    default_crate_name()
}

#[proc_macro_derive(Future)]
pub fn derive_future(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        ::core::future::Future,
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
    let (crate_, _) = crate_name(&["futures", "futures-util", "futures-core"]);

    derive_trait(
        &parse_macro_input!(input),
        parse_quote!(::#crate_::stream::Stream),
        None,
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
    .into()
}

#[proc_macro_derive(Sink)]
pub fn derive_sink(input: TokenStream) -> TokenStream {
    let (crate_, original) = crate_name(&["futures", "futures-sink"]);
    let path = if original.as_ref().map_or(false, |s| s == "futures-sink") {
        quote!(::#crate_)
    } else {
        quote!(::#crate_::sink)
    };

    derive_trait(&parse_macro_input!(input), parse_quote!(#path::Sink), None, parse_quote! {
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
    })
    .into()
}

#[proc_macro_derive(AsyncRead)]
pub fn derive_async_read(input: TokenStream) -> TokenStream {
    let (crate_, original) = crate_name(&["futures", "futures-io"]);

    let path = if original.as_ref().map_or(false, |s| s == "futures-io") {
        quote!(::#crate_)
    } else {
        quote!(::#crate_::io)
    };

    derive_trait(&parse_macro_input!(input), parse_quote!(#path::AsyncRead), None, parse_quote! {
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
    })
    .into()
}

#[proc_macro_derive(AsyncWrite)]
pub fn derive_async_write(input: TokenStream) -> TokenStream {
    let (crate_, original) = crate_name(&["futures", "futures-io"]);

    let path = if original.as_ref().map_or(false, |s| s == "futures-io") {
        quote!(::#crate_)
    } else {
        quote!(::#crate_::io)
    };

    derive_trait(&parse_macro_input!(input), parse_quote!(#path::AsyncWrite), None, parse_quote! {
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
    })
    .into()
}

#[proc_macro_derive(AsyncSeek)]
pub fn derive_async_seek(input: TokenStream) -> TokenStream {
    let (crate_, original) = crate_name(&["futures", "futures-io"]);

    let path = if original.as_ref().map_or(false, |s| s == "futures-io") {
        quote!(::#crate_)
    } else {
        quote!(::#crate_::io)
    };

    derive_trait(&parse_macro_input!(input), parse_quote!(#path::AsyncSeek), None, parse_quote! {
        trait AsyncSeek {
            #[inline]
            fn poll_seek(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
                pos: ::std::io::SeekFrom,
            ) -> ::core::task::Poll<::std::io::Result<u64>>;
        }
    })
    .into()
}

#[proc_macro_derive(AsyncBufRead)]
pub fn derive_async_buf_read(input: TokenStream) -> TokenStream {
    let (crate_, original) = crate_name(&["futures", "futures-io"]);

    let path = if original.as_ref().map_or(false, |s| s == "futures-io") {
        quote!(::#crate_)
    } else {
        quote!(::#crate_::io)
    };

    derive_trait(
        &parse_macro_input!(input),
        parse_quote!(#path::AsyncBufRead),
        None,
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
    .into()
}
