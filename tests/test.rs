#![feature(futures_api)]
#![deny(warnings)]
#![allow(dead_code)]

use futures_enum::*;

#[derive(Future, Stream, Sink, AsyncRead, AsyncWrite)]
enum Either<A, B> {
    A(A),
    B(B),
}
