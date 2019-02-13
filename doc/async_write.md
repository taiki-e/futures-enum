## [`AsyncWrite`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.12/futures/io/trait.AsyncWrite.html)

When deriving for enum like the following:

```rust
#[enum_derive(AsyncWrite)]
enum Enum<A, B> {
    A(A),
    B(B),
}
```

Code like this will be generated:

```rust
enum Enum<A, B> {
    A(A),
    B(B),
}

#[allow(unsafe_code)]
impl<A, B> ::futures::io::AsyncWrite for Enum<A, B>
where
    A: ::futures::io::AsyncWrite,
    B: ::futures::io::AsyncWrite,
{
    #[inline]
    fn poll_write(
        &mut self,
        lw: &::core::task::LocalWaker,
        buf: &[u8],
    ) -> ::core::task::Poll<::core::result::Result<usize, ::futures::io::Error>> {
        match self {
            Enum::A(x) => ::futures::io::AsyncWrite::poll_write(x, lw, buf),
            Enum::B(x) => ::futures::io::AsyncWrite::poll_write(x, lw, buf),
        }
    }

    #[inline]
    fn poll_vectored_write(
        &mut self,
        lw: &::core::task::LocalWaker,
        vec: &[&::futures::io::IoVec],
    ) -> ::core::task::Poll<::core::result::Result<usize, ::futures::io::Error>> {
        match self {
            Enum::A(x) => ::futures::io::AsyncWrite::poll_vectored_write(x, lw, vec),
            Enum::B(x) => ::futures::io::AsyncWrite::poll_vectored_write(x, lw, vec),
        }
    }

    #[inline]
    fn poll_flush(
        &mut self,
        lw: &::core::task::LocalWaker,
    ) -> ::core::task::Poll<::core::result::Result<(), ::futures::io::Error>> {
        match self {
            Enum::A(x) => ::futures::io::AsyncWrite::poll_flush(x, lw),
            Enum::B(x) => ::futures::io::AsyncWrite::poll_flush(x, lw),
        }
    }

    #[inline]
    fn poll_close(
        &mut self,
        lw: &::core::task::LocalWaker,
    ) -> ::core::task::Poll<::core::result::Result<(), ::futures::io::Error>> {
        match self {
            Enum::A(x) => ::futures::io::AsyncWrite::poll_close(x, lw),
            Enum::B(x) => ::futures::io::AsyncWrite::poll_close(x, lw),
        }
    }
}
```
