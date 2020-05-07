use futures_mockstream::*;
use futures_util::stream::StreamExt;
use futures_util::{AsyncReadExt, AsyncWriteExt};

#[test]
fn multiple_read_none() {
    let packets: &[&[u8]] = &[&[], &[]];
    let mut ms = MockStream::from(&packets[..]);
    smol::run(async {
        for i in 0..ms.len() - 1 {
            let mut buf = [0u8; 1024];
            if let Ok(p) = ms.read(&mut buf).await {
                assert_eq!(p, packets[i].len());
            }
        }
    })
}

#[test]
fn multiple_read_sized() {
    let packets: &[&[u8]] = &[&b"first packet"[..], &b"second packet"[..]];
    let mut ms = MockStream::from(&packets[..]);
    smol::run(async {
        for i in 0..ms.len() - 1 {
            let mut buf = [0u8; 1024];
            if let Ok(p) = ms.read(&mut buf).await {
                assert_eq!(&buf[..p], packets[i]);
                assert_eq!(p, packets[i].len());
            }
        }
    })
}

#[test]
fn multiple_write_none() {
    let packets: &[&[u8]] = &[&[], &[]];
    let mut ms = MockStream::default();
    smol::run(async {
        for p in packets {
            if let Ok(n) = ms.write(p).await {
                assert_eq!(n, p.len());
            }
        }
    })
}

#[test]
fn multiple_write_sized() {
    let packets: &[&[u8]] = &[&b"first packet"[..], &b"second packet"[..]];
    let mut ms = MockStream::from(&packets[..]);
    smol::run(async {
        for p in packets {
            if let Ok(n) = ms.write(p).await {
                assert_eq!(n, p.len());
            }
        }
    })
}

#[test]
fn multiple_stream_none() {
    let packets: &[&[u8]] = &[&[], &[]];
    let mut ms = MockStream::from(&packets[..]);
    smol::run(async {
        for i in 0..ms.len() - 1 {
            while let Some(p) = ms.next().await {
                assert_eq!(p.unwrap(), packets[i]);
            }
        }
    })
}

#[test]
fn multiple_stream_sized() {
    let packets: &[&[u8]] = &[&b"first packet"[..], &b"second packet"[..]];
    let mut ms = MockStream::from(&packets[..]);
    smol::run(async {
        for i in 0..ms.len() - 1 {
            while let Some(p) = ms.next().await {
                assert_eq!(p.unwrap(), packets[i]);
            }
        }
    })
}
