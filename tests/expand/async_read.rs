// SPDX-License-Identifier: Apache-2.0 OR MIT

use futures_enum::*;

#[derive(AsyncRead)]
enum Enum<A, B> {
    A(A),
    B(B),
}

fn main() {}
