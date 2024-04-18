use std::{fs, io};
use std::io::Write;

use flate2::write::ZlibDecoder;

use crate::byte_stream::ByteStream;
use crate::unbundled_file::UnbundledFile;
use crate::unbundled_file_creator::UnbundledFileCreator;

#[derive(Debug)]
pub enum UnbundlerError {
    IOError(io::Error),
    DecoderFinish,
    DecoderWriteAll,
}

//TODO: impl From for exceptions? see lumi msg.
pub struct Unbundler {
    compressed_stream: ByteStream,
}

impl<'a> Unbundler {
    pub fn new(compressed_file_path: &'a str) -> Result<Unbundler, UnbundlerError> {
        let file = match fs::read(compressed_file_path) {
            Ok(file) => file,
            Err(e) => return Err(UnbundlerError::IOError(e)),
        };
        Ok(Unbundler {
            compressed_stream: ByteStream::new(file),
        })
    }

    pub fn unbundle_file(&mut self, dds_mode: bool) -> Result<Vec<UnbundledFile>, UnbundlerError> {
        let mut unbundled_files: Vec<UnbundledFile> = vec![];

        let inflated_bundle = match self.inflate_bundle() {
            Ok(inflated_bundle) => inflated_bundle,
            Err(e) => return Err(e),
        };

        let mut inflated_stream = ByteStream::new(inflated_bundle);

        let file_count = inflated_stream.read_uint();
        let _checksum = inflated_stream.read(256);
        let _file_names_and_extensions = inflated_stream.read((16 * file_count) as usize);

        let file_creator = UnbundledFileCreator::new(dds_mode);

        for _i in 0..file_count {
            let unbundled_file = file_creator.create(&mut inflated_stream);
            unbundled_files.push(unbundled_file);
        }

        Ok(unbundled_files) 
    }

    pub fn inflate_bundle(&mut self) -> Result<Vec<u8>, UnbundlerError> {
        let _header = self.compressed_stream.read_uint();
        let _size = self.compressed_stream.read_uint();
        let _reserved = self.compressed_stream.read_uint();

        let mut result: Vec<u8> = vec![];

        while self.compressed_stream.remaining_bytes() > 0 {
            match self.append_block(&mut result) {
                Ok(_) => {},
                Err(e) => return Err(e),
            }
        }

        Ok(result)
    }

    fn append_block(&mut self, buffer: &mut Vec<u8>) -> Result<(), UnbundlerError> {
        let len = self.compressed_stream.read_uint();
        if len == (1<<16) {
            buffer.append(&mut self.compressed_stream.read(len as usize))
        } else {
            let mut block = match self.decompress_block(len as usize) {
                Ok(block) => block,
                Err(e) => return Err(e),
            };
            buffer.append(&mut block);
        }
        Ok(())
    }

    fn decompress_block(&mut self, len: usize) -> Result<Vec<u8>, UnbundlerError> {
        let mut decoder = ZlibDecoder::new(vec![]);

        match decoder.write_all(&self.compressed_stream.read(len as usize)) {
            Ok(_) => {},
            Err(_) => return Err(UnbundlerError::DecoderWriteAll),
        }

        match decoder.finish() {
            Ok(block) => Ok(block),
            Err(_) => Err(UnbundlerError::DecoderFinish),
        }
    }
}