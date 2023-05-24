use crate::byte_stream::ByteStream;

#[derive(Clone)]
pub struct UnbundledFile {
    pub path: u64,
    pub extension: u64,
    pub data: Vec<u8>,
}

impl UnbundledFile {
    pub fn new(inflated_stream: &mut ByteStream) -> UnbundledFile {
        let extension = inflated_stream.read_ulong();
        let path = inflated_stream.read_ulong();
        let has_data = inflated_stream.read_ulong();

        let data;
        if has_data > 0 {
            let _flag = inflated_stream.read_uint();
            let size = inflated_stream.read_uint();
            let _unknown2 = inflated_stream.read_uint();
            data = inflated_stream.read(size as usize);
        } else {
            data = vec![];
        }

        UnbundledFile {
            extension: extension,
            path: path,
            data: data,
        }
    }
}