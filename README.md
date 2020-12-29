# futures-enum

[![crates.io](https://img.shields.io/crates/v/futures-enum.svg?style=flat-square&logo=rust)](https://crates.io/crates/futures-enum)
[![docs.rs](https://img.shields.io/badge/docs.rs-futures--enum-blue?style=flat-square)](https://docs.rs/futures-enum)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg?style=flat-square)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.36+-blue.svg?style=flat-square)](https://www.rust-lang.org)
[![build status](https://img.shields.io/github/workflow/status/taiki-e/futures-enum/CI/master?style=flat-square)](https://github.com/taiki-e/futures-enum/actions?query=workflow%3ACI+branch%3Amaster)

\#\[derive(Future, Stream, Sink, AsyncRead, AsyncWrite, AsyncSeek, AsyncBufRead)\] for enums.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
futures-enum = "0.1.16"
futures = "0.3"
```

*Compiler support: requires rustc 1.36+*

## Examples

```rust
use futures_enum::*;
use std::future::Future;

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

See [auto_enums](https://github.com/taiki-e/auto_enums) crate for how to
automate patterns like this.

## Supported traits

* [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html) - [generated code](doc/future.md)
* [`Stream`](https://docs.rs/futures/0.3/futures/stream/trait.Stream.html) - [generated code](doc/stream.md)
* [`Sink`](https://docs.rs/futures/0.3/futures/sink/trait.Sink.html) - [generated code](doc/sink.md)
* [`AsyncRead`](https://docs.rs/futures/0.3/futures/io/trait.AsyncRead.html) - [generated code](doc/async_read.md)
* [`AsyncWrite`](https://docs.rs/futures/0.3/futures/io/trait.AsyncWrite.html) - [generated code](doc/async_write.md)
* [`AsyncSeek`](https://docs.rs/futures/0.3/futures/io/trait.AsyncSeek.html) - [generated code](doc/async_seek.md)
* [`AsyncBufRead`](https://docs.rs/futures/0.3/futures/io/trait.AsyncBufRead.html) - [generated code](doc/async_buf_read.md)

## Related Projects

* [auto_enums]: A library for to allow multiple return types by automatically generated enum.
* [derive_utils]: A procedural macro helper for easily writing [derives macros][proc-macro-derive] for enums.
* [io-enum]: \#\[derive(Read, Write, Seek, BufRead)\] for enums.
* [iter-enum]: \#\[derive(Iterator, DoubleEndedIterator, ExactSizeIterator, Extend)\] for enums.

[auto_enums]: https://github.com/taiki-e/auto_enums
[derive_utils]: https://github.com/taiki-e/derive_utils
[io-enum]: https://github.com/taiki-e/io-enum
[iter-enum]: https://github.com/taiki-e/iter-enum

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
