use futures_enum::*;

#[derive(AsyncSeek)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
