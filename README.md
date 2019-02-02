# futures-enum

[![Build Status](https://travis-ci.com/taiki-e/futures-enum.svg?branch=master)](https://travis-ci.com/taiki-e/futures-enum)
[![version](https://img.shields.io/crates/v/futures-enum.svg)](https://crates.io/crates/futures-enum/)
[![documentation](https://docs.rs/futures-enum/badge.svg)](https://docs.rs/futures-enum/)
[![license](https://img.shields.io/crates/l/futures-enum.svg)](https://crates.io/crates/futures-enum/)

\#\[derive(Future, Stream, Sink, AsyncRead, AsyncWrite)\] for enums.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
futures-enum = "0.1.0"
```

Now, you can use futures-enum:

```rust
use futures_enum::*;
```

The current version of futures-enum requires Rust nightly 2019-01-11 or later.

## Examples

```rust
use futures_enum::*;

#[derive(Future, Stream, Sink, AsyncRead, AsyncWrite)]
enum Either<A, B> {
    A(A),
    B(B),
}

#[derive(Future, Stream, Sink, AsyncRead, AsyncWrite)]
enum Either3<A, B, C> {
    A(A),
    B(B),
    C(C),
}
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
