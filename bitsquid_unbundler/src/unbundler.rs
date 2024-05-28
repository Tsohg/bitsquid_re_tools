use std::path::PathBuf;
use std::{fs, io};
use std::io::Write;

use flate2::write::ZlibDecoder;

use crate::byte_stream::ByteStream;
use crate::unbundled_directory::UnbundledDirectory;
use crate::unbundled_file::UnbundledFile;
use crate::extensions::Extensions;

#[derive(Debug)]
pub enum UnbundlerError {
    IOError(io::Error),
    DecoderFinish,
    DecoderWriteAll,
    Inflater,
    Extension,
}

pub struct Unbundler {
    file_path: PathBuf,
    dds_mode: bool,
}

impl Unbundler {
    pub fn new(file_path: PathBuf, dds_mode: bool) -> Unbundler {
        Unbundler {
            file_path,
            dds_mode,
        }
    }

    fn has_valid_extension(path: &PathBuf) -> bool {      
        match path.extension() {
            Some(ext) => match ext.to_str().unwrap() {
                "stream" | "ini" | "data" => false,
                _ => true,
            },
            None => true,
        }
    }
    
    pub fn unbundle(&mut self) -> Result<Vec<UnbundledDirectory>, UnbundlerError>{
        let mut unbundled_dirs = vec![];

        if !Unbundler::has_valid_extension(&self.file_path) { return Err(UnbundlerError::Extension) }

        if self.file_path.is_file() {
            let file_path = String::from(self.file_path.to_str().unwrap());
            let file_name = String::from(self.file_path.file_name().unwrap().to_str().unwrap());
            let unbundled = self.parse_bundle(&file_name, &file_path).unwrap();
            unbundled_dirs.push(unbundled);
            return Ok(unbundled_dirs);
        }

        let read_dir = self.file_path.read_dir().unwrap();

        for entry in read_dir {
            let file_path = String::from(entry.as_ref().unwrap().path().to_str().unwrap());
            let file_name = String::from(entry.as_ref().unwrap().file_name().to_str().unwrap());
            let unbundled = self.parse_bundle(&file_name, &file_path).unwrap();
            unbundled_dirs.push(unbundled);
        }

        Ok(unbundled_dirs)
    }

    fn parse_bundle(&self, file_name: &String, file_path: &String) -> Result<UnbundledDirectory, UnbundlerError> {
        let file = match fs::read(file_path) {
            Ok(file) => file,
            Err(e) => return Err(UnbundlerError::IOError(e)),
        };

        let mut compressed_stream = ByteStream::new(file);

        match self.inflate_file(&mut compressed_stream) {
            Ok(files) => Ok(UnbundledDirectory::new(String::from(file_name), files)),
            Err(e) => { 
                print!("Encountered an error unbundling file: {}\n{:?}", file_path, e);
                return Err(UnbundlerError::Inflater);
            },
        }
    }

    pub fn inflate_file(&self, compressed_stream: &mut ByteStream) -> Result<Vec<UnbundledFile>, UnbundlerError> {
        let mut unbundled_files: Vec<UnbundledFile> = vec![];

        let inflated_bundle = match self.inflate_bundle(compressed_stream) {
            Ok(inflated_bundle) => inflated_bundle,
            Err(e) => return Err(e),
        };

        let mut inflated_stream = ByteStream::new(inflated_bundle);

        let file_count = inflated_stream.read_uint();
        let _checksum = inflated_stream.read(256);
        let _file_names_and_extensions = inflated_stream.read((16 * file_count) as usize);

        for _i in 0..file_count {          
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
            
            unbundled_files.push(UnbundledFile {
                extension: Extensions::lookup(extension, self.dds_mode),
                path,
                data,
            });
        }

        Ok(unbundled_files)
    }

    pub fn inflate_bundle(&self, compressed_stream: &mut ByteStream) -> Result<Vec<u8>, UnbundlerError> {
        let _header = compressed_stream.read_uint();
        let _size = compressed_stream.read_uint();
        let _reserved = compressed_stream.read_uint();

        let mut result: Vec<u8> = vec![];

        while compressed_stream.remaining_bytes() > 0 {
            match self.append_block(compressed_stream, &mut result) {
                Ok(_) => {},
                Err(e) => return Err(e),
            }
        }

        Ok(result)
    }

    fn append_block(&self, compressed_stream: &mut ByteStream, buffer: &mut Vec<u8>) -> Result<(), UnbundlerError> {
        let len = compressed_stream.read_uint();
        if len == (1<<16) {
            buffer.append(&mut compressed_stream.read(len as usize))
        } else {
            let mut block = match self.decompress_block(compressed_stream, len as usize) {
                Ok(block) => block,
                Err(e) => return Err(e),
            };
            buffer.append(&mut block);
        }
        Ok(())
    }

    fn decompress_block(&self, compressed_stream: &mut ByteStream, len: usize) -> Result<Vec<u8>, UnbundlerError> {
        let mut decoder = ZlibDecoder::new(vec![]);

        match decoder.write_all(&compressed_stream.read(len as usize)) {
            Ok(_) => {},
            Err(_) => return Err(UnbundlerError::DecoderWriteAll),
        }

        match decoder.finish() {
            Ok(block) => Ok(block),
            Err(_) => Err(UnbundlerError::DecoderFinish),
        }
    }
}