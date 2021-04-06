use futures_enum::*;

#[derive(Future)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
