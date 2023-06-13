pub struct ByteStream {
    position: usize,
    bytes: Vec<u8>,
}

impl ByteStream {
    pub fn new(bytes: Vec<u8>) -> ByteStream {
        ByteStream { 
            position: 0,
            bytes: bytes,
         }
    }

    pub fn read_byte(&mut self) -> u8 {
        self.position += 1;
        self.bytes[self.position - 1]
    }

    pub fn read_uint(&mut self) -> u32 {
        (self.read_byte() as u32) |
        (self.read_byte() as u32) << 8 |
        (self.read_byte() as u32) << 16 |
        (self.read_byte() as u32) << 24
    }

    pub fn read_ulong(&mut self) -> u64 {
        (self.read_uint() as u64) | 
        (self.read_uint() as u64) << 32
    }

    pub fn read(&mut self, len: usize) -> Vec<u8> {
        let mut result = vec![];
        
        for _i in 0..len {
            result.push(self.read_byte());
        }

        result
    }

    pub fn remaining_bytes(&self) -> usize {
        self.bytes.len() - self.position
    }
}

#[cfg(test)]
mod tests {
    use super::ByteStream;
    #[test]
    fn read_byte()
    {
        let mut stream = ByteStream::new(vec![0x1]);
        assert_eq!(0x1, stream.read_byte());
    }

    #[test]
    fn read_two_bytes() {
        let mut stream = 
            ByteStream::new(vec![0x1, 0x4]);
        assert_eq!(0x1, stream.read_byte());
        assert_eq!(0x4, stream.read_byte());
    }

    #[test]
    #[should_panic]
    fn read_byte_from_empty_vector_panics() {
        let mut stream = 
            ByteStream::new(vec![]);
        stream.read_byte();
    }

    #[test]
    fn read_uint() {
        let mut stream = 
            ByteStream::new(Vec::from(32_u32.to_le_bytes()));
        assert_eq!(32_u32, stream.read_uint());
    }

    #[test]
    fn read_two_uints() {
        let mut vec = Vec::from(32_u32.to_le_bytes());
        let mut vec2 = Vec::from(42_u32.to_le_bytes());
        vec.append(&mut vec2);

        let mut stream = ByteStream::new(vec);
        assert_eq!(32_u32, stream.read_uint());
        assert_eq!(42_u32, stream.read_uint());
    }

    #[test]
    fn read_ulong() {
        let mut stream = 
            ByteStream::new(Vec::from(12345633_u64.to_le_bytes()));
        assert_eq!(12345633_u64, stream.read_ulong());
    }

    #[test]
    fn read() {
        let vec = vec![1,2,3,4,5,6,7,8,9,1,2,3,4,5,6,7,8,9];
        let expected = vec.clone();

        let mut stream = ByteStream::new(vec);
        assert_eq!(expected, stream.read(expected.len()));
    }

    #[test]
    fn remaining_bytes() {
        let vec = vec![1,2,3,4];
        let mut stream = ByteStream::new(vec);
        assert_eq!(4, stream.remaining_bytes());
        stream.read(1);
        assert_eq!(3, stream.remaining_bytes());
        stream.read(3);
        assert_eq!(0, stream.remaining_bytes());
    }
}