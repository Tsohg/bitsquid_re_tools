pub struct ByteStream {
    position: usize,
    bytes: Vec<u8>,
}

impl ByteStream {
    pub fn new(bytes: Vec<u8>) -> ByteStream {
        ByteStream {
            position: 0,
            bytes,
        }
    }
    pub fn read_byte(&mut self) -> u8 {
        self.position += 1;
        self.bytes[self.position - 1]
    }

    pub fn read_uint(&mut self) -> u32 {
        (self.read_byte() as u32)
            | (self.read_byte() as u32) << 8
            | (self.read_byte() as u32) << 16
            | (self.read_byte() as u32) << 24
    }

    pub fn read_ulong(&mut self) -> u64 {
        (self.read_uint() as u64) | (self.read_uint() as u64) << 32
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

    pub fn peek_byte(&mut self) -> u8 {
        self.bytes[self.position]
    }

    pub fn peek_bytes(&mut self, n: usize) -> Vec<u8> {
        self.bytes[self.position..n].to_vec()
    }

    pub fn read_uleb(&mut self) -> u32 {
        let mut value: u32 = 0;
        let mut shift: Option<u32> = Some(1);
        loop {
            let byte = self.read_byte();
            let data = byte & 127u8;
            let cont = byte & 128u8;
            value += data as u32 * shift.unwrap();
            shift = shift.unwrap().checked_mul(128);
            if cont == 0 {
                break;
            }
        }
        value
    }
}