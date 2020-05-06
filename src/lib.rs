//! Futures MockStream gives you a MockStream for testing your custom AsyncRead, AsyncWrite and
//! Streams implementations.
//!
//! # Examples
//! ```compile_fail
//!# use futures_mockstream::MockStream;
//!struct MyConn<S> {
//!     stream: S,
//!}
//!
//!let mut ms = MockStream::from(&b"GET /index HTTP/1.1\r\n");
//!smol::run(async {
//!     while let Some(item) = MyConn::new(&mut ms).next().await {
//!         println!("{}", item);
//!     }
//!})
//! ```
use futures_core::Stream;
use futures_io::{AsyncRead, AsyncWrite};
use futures_task::{Context, Poll};
use std::io::{self, Cursor, Read, Write};
use std::pin::Pin;

#[cfg(test)]
mod tests;

/// A Mock Stream with implements AsyncRead, AsyncWrite, and Stream from the futures crate.
///
/// # Examples
/// ```
/// # use futures_mockstream::MockStream;
/// let mock_stream = MockStream::from(&b"mock stream buffer"[..]);
/// ```
#[derive(Default, Debug)]
pub struct MockStream {
    buf: Cursor<Vec<u8>>,
    from_index: usize,
}

impl Unpin for MockStream {}

impl MockStream {
    /// Creates a MockStream from a reference of array.
    ///
    /// # Arguments
    /// A reference of array of u8.
    ///
    /// # Examples
    /// ```
    /// # use futures_mockstream::MockStream;
    /// let mockstream = MockStream::from("hello".as_bytes());
    /// ```
    pub fn from(buf: &[u8]) -> Self {
        Self {
            buf: Cursor::new(Vec::from(buf)),
            from_index: 0,
        }
    }
}

impl AsyncRead for MockStream {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let this: &mut Self = Pin::into_inner(self);
        Poll::Ready(this.buf.read(buf))
    }
}

impl AsyncWrite for MockStream {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let this: &mut Self = Pin::into_inner(self);
        Poll::Ready(this.buf.write(buf))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

impl Stream for MockStream {
    type Item = Result<Vec<u8>, io::Error>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this: &mut Self = Pin::into_inner(self);
        let mut buf = [0u8; 1024];
        match Pin::new(this).poll_read(cx, &mut buf) {
            Poll::Pending => Poll::Ready(None),
            Poll::Ready(Ok(b)) if b == 0 => Poll::Ready(None),
            Poll::Ready(Ok(b)) => Poll::Ready(Some(Ok(Vec::from(&buf[..b])))),
            Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
        }
    }
}

impl AsRef<[u8]> for MockStream {
    fn as_ref(&self) -> &[u8] {
        self.buf.get_ref()
    }
}
