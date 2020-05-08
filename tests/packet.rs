use futures_mockstream::Packet;
use std::io::Write;

#[test]
fn packet_flush() {
    let mut packet = Packet::default();
    let buf = b"this is the packet";
    let written = packet.write(&buf[..]).expect("failed to write to packet");
    let _ = packet.flush().expect("failed to flush packet");
    assert_eq!(written, packet.as_ref().len());
}
