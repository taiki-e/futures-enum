## [`AsyncWrite`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/io/trait.AsyncWrite.html)

When deriving for enum like the following:

```rust
#[derive(AsyncWrite)]
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
        cx: &::core::task::Context<'_>,
        buf: &[u8],
    ) -> ::core::task::Poll<::core::result::Result<usize, ::futures::io::Error>> {
        match self {
            Enum::A(x) => ::futures::io::AsyncWrite::poll_write(x, cx, buf),
            Enum::B(x) => ::futures::io::AsyncWrite::poll_write(x, cx, buf),
        }
    }

    #[inline]
    fn poll_vectored_write(
        &mut self,
        cx: &::core::task::Context<'_>,
        vec: &[&::futures::io::IoVec],
    ) -> ::core::task::Poll<::core::result::Result<usize, ::futures::io::Error>> {
        match self {
            Enum::A(x) => ::futures::io::AsyncWrite::poll_vectored_write(x, cx, vec),
            Enum::B(x) => ::futures::io::AsyncWrite::poll_vectored_write(x, cx, vec),
        }
    }

    #[inline]
    fn poll_flush(
        &mut self,
        cx: &::core::task::Context<'_>,
    ) -> ::core::task::Poll<::core::result::Result<(), ::futures::io::Error>> {
        match self {
            Enum::A(x) => ::futures::io::AsyncWrite::poll_flush(x, cx),
            Enum::B(x) => ::futures::io::AsyncWrite::poll_flush(x, cx),
        }
    }

    #[inline]
    fn poll_close(
        &mut self,
        cx: &::core::task::Context<'_>,
    ) -> ::core::task::Poll<::core::result::Result<(), ::futures::io::Error>> {
        match self {
            Enum::A(x) => ::futures::io::AsyncWrite::poll_close(x, cx),
            Enum::B(x) => ::futures::io::AsyncWrite::poll_close(x, cx),
        }
    }
}
```
