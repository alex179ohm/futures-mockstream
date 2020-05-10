use futures_mockstream::*;
use futures_util::stream::StreamExt;
use futures_util::{AsyncReadExt, AsyncWriteExt};

#[allow(clippy::needless_range_loop)]
#[test]
fn read_empty() {
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

#[allow(clippy::needless_range_loop)]
#[test]
fn read_sized() {
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
fn write_empty() {
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
fn write_sized() {
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
fn stream_empty() {
    let packets: &[&[u8]] = &[&[], &[]];
    let mut ms = MockStream::from(&packets[..]);
    smol::run(async {
        let mut index = 0;
        while let Some(p) = ms.next().await {
            assert_eq!(p.unwrap(), packets[index]);
            index += 1;
        }
    })
}

#[test]
fn stream_sized() {
    let packets: &[&[u8]] = &[&b"first packet"[..], &b"second packet"[..]];
    let mut ms = MockStream::from(&packets[..]);
    smol::run(async {
        let mut index = 0;
        while let Some(p) = ms.next().await {
            assert_eq!(p.unwrap(), packets[index]);
            index += 1;
        }
    })
}

#[allow(clippy::needless_range_loop)]
#[test]
fn flush_empty() {
    let packets: &[&[u8]] = &[&[], &[]];
    let mut ms = MockStream::default();
    smol::run(async {
        for i in 0..packets.len() - 1 {
            let _ = ms.write(packets[i]).await;
            let _ = ms.flush().await;
            assert_eq!(packets[i].len(), ms.as_ref().len());
            assert_eq!(packets[i], ms.as_ref());
            assert_eq!(i + 1, ms.len());
        }
    })
}

#[allow(clippy::needless_range_loop)]
#[test]
fn flush_sized() {
    let packets: &[&[u8]] = &[&b"first packet"[..], &b"second packet"[..]];
    let mut ms = MockStream::default();
    smol::run(async {
        for i in 0..packets.len() - 1 {
            let _ = ms.write(packets[i]).await;
            let _ = ms.flush().await;
            assert_eq!(packets[i].len(), ms.as_ref().len());
            assert_eq!(packets[i], ms.as_ref());
            assert_eq!(i + 1, ms.len());
        }
    })
}
