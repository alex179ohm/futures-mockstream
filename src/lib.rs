//! Futures MockStream gives you a MockStream for testing your custom AsyncRead, AsyncWrite and
//! Streams implementations.
//!
//! # Examples
//! ```no-run
//!# use futures_mockstream::MockStream;
//!let mut ms = MockStream::from(&b"GET /index HTTP/1.1\r\n");
//!smol::run(async {
//!     for item in MyStream.read(&mut ms).next().await {
//!         println!("{}", item);
//!     }
//!})
//! ```
use futures_core::Stream;
use futures_io::{AsyncRead, AsyncWrite};
use futures_task::{Context, Poll};
use std::io;
use std::io::Cursor;
use std::io::{Read, Write};
use std::io::{Seek, SeekFrom};
use std::pin::Pin;

/// A Mock Stream with implements AsyncRead, AsyncWrite, and Stream from the futures crate.
///
/// # Examples
/// ```
/// # use futures_mockstream::MockStream;
/// let mock_stream = MockStream::from(&b"mock stream buffer"[..]);
/// ```
#[derive(Default, Clone)]
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
        this.buf
            .seek(SeekFrom::Start(this.from_index as u64))
            .unwrap();
        match this.buf.read(buf) {
            Ok(readed) => {
                this.from_index += readed;
                Poll::Ready(Ok(readed))
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

impl AsyncWrite for MockStream {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let this: &mut Self = Pin::into_inner(self);
        match this.buf.write(buf) {
            Ok(written) => Poll::Ready(Ok(written)),
            Err(e) => Poll::Ready(Err(e)),
        }
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
        let content = this.clone();
        match Pin::new(this).poll_read(cx, &mut buf) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(b)) if b == 0 => Poll::Ready(None),
            Poll::Ready(Ok(_)) => Poll::Ready(Some(Ok(Vec::from(content.as_ref())))),
            Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
        }
    }
}

impl AsRef<[u8]> for MockStream {
    fn as_ref(&self) -> &[u8] {
        self.buf.get_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::stream::StreamExt;
    use futures_util::{AsyncReadExt, AsyncWriteExt};

    #[test]
    fn async_read() {
        let mut ms = MockStream::default();
        let mut buf = [0u8; 1024];
        smol::run(async {
            let readed = ms.read(&mut buf).await.expect("failed to read");
            assert_eq!(readed, 0);
        })
    }

    #[test]
    fn async_read_sized() {
        let packet = b"ciao mondo";
        let mut ms = MockStream::from(packet);
        smol::run(async {
            let mut buf = [0u8; 1024];
            let readed = ms.read(&mut buf).await.expect("failed to read");
            assert_eq!(readed, 10);
            assert_eq!(&b"ciao mondo"[..], &buf[..readed]);
        })
    }

    #[test]
    fn async_write() {
        let buf = &[];
        let mut ms = MockStream::default();
        smol::run(async {
            let written = ms.write(buf).await.expect("failed to write");
            assert_eq!(written, 0);
        })
    }

    #[test]
    fn async_write_sized() {
        let buf = b"this is the packet";
        let mut ms = MockStream::default();
        smol::run(async {
            let written = ms.write(buf).await.expect("failed to write");
            assert_ne!(written, 0);
            assert_eq!(&buf[..], ms.as_ref());
        })
    }

    #[test]
    fn async_stream_none() {
        let buf: &[u8] = &[];
        let mut ms = MockStream::from(&buf);
        smol::run(async {
            if let Some(v) = ms.next().await {
                match v {
                    Ok(b) => assert_eq!(b.len(), 0),
                    Err(e) => panic!("{}", e),
                }
            }
        })
    }

    #[test]
    fn async_stream_sized() {
        let buf = b"this is my packet";
        let mut ms = MockStream::from(buf);
        smol::run(async {
            if let Some(v) = ms.next().await {
                match v {
                    Ok(b) => {
                        assert_eq!(b.len(), buf.len());
                        assert_eq!(&buf[..], buf.as_ref());
                    }
                    Err(e) => panic!("{}", e),
                }
            }
        })
    }
}
