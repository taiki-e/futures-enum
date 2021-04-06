use futures_enum::*;

#[derive(AsyncBufRead)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
