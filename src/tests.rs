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
        dbg!(ms);
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
        while let Some(v) = ms.next().await {
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
        while let Some(v) = ms.next().await {
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
