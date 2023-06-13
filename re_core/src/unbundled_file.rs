#[derive(Clone)]
pub struct UnbundledFile {
    pub path: u64, //u64 path because it is a hash.
    pub extension: String,
    pub data: Vec<u8>,
}