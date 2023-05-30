#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(dead_code)]

use core::future::Future;

use futures::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite, Sink, Stream};
use futures_enum::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite, Future, Sink, Stream};

#[derive(Future, Stream, Sink, AsyncRead, AsyncWrite, AsyncSeek, AsyncBufRead)]
enum Either<A, B> {
    A(A),
    B(B),
}

fn _assert_impl<
    T: Future + Stream + Sink<()> + AsyncRead + AsyncWrite + AsyncSeek + AsyncBufRead,
>() {
    fn __assert_impl<
        T: Future + Stream + Sink<()> + AsyncRead + AsyncWrite + AsyncSeek + AsyncBufRead,
    >() {
    }
    __assert_impl::<Either<T, T>>();
}
