use std::{
    fs::File,
    io::{Read, Seek, SeekFrom}
};

pub struct LJFileReader {
    file: File,
}
impl LJFileReader {
    pub fn new(file_path: &str) -> LJFileReader {
        LJFileReader {
            file: File::open(file_path).expect("File could not be opened."),
        }
    }

    ///Reads one byte from the file stream and advances the stream position.
    pub fn read_byte(&mut self) -> u8 {
        let mut buf = [0u8; 1];
        self.file
            .read_exact(&mut buf)
            .expect("File could not read byte.");
        buf[0]
    }

    ///Reads n bytes from the file stream and advances the stream position by n.
    pub fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![0u8; n];
        self.file
            .read_exact(&mut buf)
            .expect(&format!("File could not read {} bytes.", n));
        buf
    }

    ///Reads a byte from the file stream then returns to its original stream position.
    pub fn peek_byte(&mut self) -> u8 {
        let pos = self.file.stream_position().unwrap();
        let b = self.read_byte();
        self.file.seek(SeekFrom::Start(pos)).unwrap();
        b
    }

    ///Reads n bytes from the file stream then returns to its original stream position.
    pub fn peek_bytes(&mut self, n: usize) -> Vec<u8> {
        let pos = self.file.stream_position().unwrap();
        let bytes = self.read_bytes(n);
        self.file.seek(SeekFrom::Start(pos)).unwrap();
        bytes
    }

    ///Returns the remaining bytes from the file stream.
    pub fn remaining_bytes(&mut self) -> u64 {
        let pos = self.file.stream_position().unwrap();
        self.file.seek(SeekFrom::End(0)).unwrap();
        let end = self.file.stream_position().unwrap();
        self.file.seek(SeekFrom::Start(pos)).unwrap();
        end - pos
    }

    ///Advances the stream position until it finds 0x1b4c4a01 which is the luajit file format identifier (ffi).
    pub fn seek_to_lj_magic(&mut self) {
        let ffi = [0x1b, 0x4c, 0x4a, 0x01];
        loop {
            let bytes = self.peek_bytes(4);
            if bytes == ffi { break; }
            self.read_bytes(4);
        } //Will panic if the ffi is not found before EOF.
    }

    pub fn read_uleb(&mut self) -> u32 {
        let mut value: u32 = 0;
        let mut shift: Option<u32> = Some(1);
        loop {
            let byte = self.read_byte();
            let data = byte & 127u8;
            let cont = byte & 128u8;
            value += data as u32 * shift.unwrap();
            shift = shift.unwrap().checked_mul(128); // <-- This can overflow on big ulebs...
            if cont == 0 { break; }
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_byte() {
        let mut r = LJFileReader::new("singleif.ljc");

        let b = r.read_byte();
        assert!(b == 0x1b);
        
        let b = r.read_byte();
        assert!(b == 0x4c);

        let b = r.read_byte();
        assert!(b == 0x4a);

        let b = r.read_byte();
        assert!(b == 0x01);
    }

    #[test]
    fn test_read_bytes() {
        let mut r = LJFileReader::new("singleif.ljc");

        let bytes = r.read_bytes(4);
        assert!(bytes == [0x1b, 0x4c, 0x4a, 0x01], "actual: {:?}", bytes);

        let bytes = r.read_bytes(4);
        assert!(bytes == [0x02, 0x65, 0x02, 0x00], "actual: {:?}", bytes);
    }

    #[test]
    fn test_peek_byte() {
        let mut r = LJFileReader::new("singleif.ljc");
        let b = r.peek_byte();
        assert!(b == 0x1b);
        let b = r.peek_byte();
        assert!(b == 0x1b);
    }

    #[test]
    fn test_peek_bytes() {
        let mut r = LJFileReader::new("singleif.ljc");

        let bytes = r.peek_bytes(4);
        assert!(bytes == [0x1b, 0x4c, 0x4a, 0x01], "actual: {:02x?}", bytes);

        let bytes = r.peek_bytes(4);
        assert!(bytes == [0x1b, 0x4c, 0x4a, 0x01], "actual: {:02x?}", bytes);
    }

    #[test]
    fn test_seek_lj_magic() {
        let mut r = LJFileReader::new("singleif.ljc.junk");
        r.seek_to_lj_magic();
        let bytes = r.peek_bytes(4);
        assert!(bytes == [0x1b, 0x4c, 0x4a, 0x01], "actual: {:02x?}", bytes);
    }

    #[test]
    fn test_remaining_bytes() {
        let mut r = LJFileReader::new("singleif.ljc");
        let remaining = r.remaining_bytes();
        assert!(remaining == 0x6c);
        let bytes = r.peek_bytes(4);
        assert!(bytes == [0x1b, 0x4c, 0x4a, 0x01], "actual: {:02x?}", bytes);
    }
}