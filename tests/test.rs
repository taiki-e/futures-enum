#![feature(futures_api)]
#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![allow(dead_code)]

use futures_enum::*;

#[derive(Future, Stream, Sink, AsyncRead, AsyncWrite)]
enum Either<A, B> {
    A(A),
    B(B),
}
