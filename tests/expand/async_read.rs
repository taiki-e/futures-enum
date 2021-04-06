use futures_enum::*;

#[derive(AsyncRead)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
