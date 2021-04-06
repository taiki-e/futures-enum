use futures_enum::*;

#[derive(Sink)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
