#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(dead_code)]

use futures_enum::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite, Future, Sink, Stream};

#[derive(Future, Stream, Sink, AsyncRead, AsyncWrite, AsyncSeek, AsyncBufRead)]
enum Either<A, B> {
    A(A),
    B(B),
}
