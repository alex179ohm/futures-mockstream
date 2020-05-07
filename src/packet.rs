use std::io::{self, Cursor, Read, Write};

#[derive(Debug, Default)]
pub struct Packet {
    buffer: Cursor<Vec<u8>>,
}

impl From<&[u8]> for Packet {
    fn from(buf: &[u8]) -> Self {
        Packet {
            buffer: Cursor::new(Vec::from(buf)),
        }
    }
}

impl Read for Packet {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.buffer.read(buf)
    }
}

impl Write for Packet {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush()
    }
}

impl AsRef<[u8]> for Packet {
    fn as_ref(&self) -> &[u8] {
        self.buffer.get_ref()
    }
}
