#![warn(unsafe_code)]
#![warn(rust_2018_idioms)]
#![allow(dead_code)]

use futures_enum::*;

#[derive(Future, Stream, Sink, AsyncRead, AsyncWrite, AsyncSeek, AsyncBufRead)]
enum Either<A, B> {
    A(A),
    B(B),
}
