# futures-enum

[![Build Status][azure-badge]][azure-url]
[![Crates.io][crates-version-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![License][crates-license-badge]][crates-url]
[![Minimum supported Rust version][rustc-badge]][rustc-url]

[azure-badge]: https://dev.azure.com/taiki-e/taiki-e/_apis/build/status/taiki-e.futures-enum?branchName=master
[azure-url]: https://dev.azure.com/taiki-e/taiki-e/_build/latest?definitionId=7&branchName=master
[crates-version-badge]: https://img.shields.io/crates/v/futures-enum.svg
[crates-license-badge]: https://img.shields.io/crates/l/futures-enum.svg
[crates-badge]: https://img.shields.io/crates/v/futures-enum.svg
[crates-url]: https://crates.io/crates/futures-enum/
[docs-badge]: https://docs.rs/futures-enum/badge.svg
[docs-url]: https://docs.rs/futures-enum/
[rustc-badge]: https://img.shields.io/badge/rustc-1.36+-lightgray.svg
[rustc-url]: https://blog.rust-lang.org/2019/07/04/Rust-1.36.0.html

\#\[derive(Future, Stream, Sink, AsyncRead, AsyncWrite, AsyncSeek, AsyncBufRead)\] for enums.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
futures-enum = "0.1.11"
futures-preview = "0.3.0-alpha.19"
```

The current futures-enum requires Rust 1.36 or later.

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

futures-enum works well even if the dependency contains only sub-crates such as `futures-core`, `futures-io`, `futures-sink`, etc.

See [auto_enums](https://github.com/taiki-e/auto_enums) for how to automate patterns like this.

## Supported traits

* [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html) - [generated code](doc/future.md)
* [`Stream`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/stream/trait.Stream.html) - [generated code](doc/stream.md)
* [`Sink`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/sink/trait.Sink.html) - [generated code](doc/sink.md)
* [`AsyncRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/io/trait.AsyncRead.html) - [generated code](doc/async_read.md)
* [`AsyncWrite`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/io/trait.AsyncWrite.html) - [generated code](doc/async_write.md)
* [`AsyncSeek`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/io/trait.AsyncSeek.html) - [generated code](doc/async_seek.md)
* [`AsyncBufRead`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/io/trait.AsyncBufRead.html) - [generated code](doc/async_buf_read.md)

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
