use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};

use flate2::write::ZlibDecoder;

use crate::byte_stream::ByteStream;
use crate::extensions::Extensions;
use crate::unbundled_directory::UnbundledDirectory;
use crate::unbundled_file::UnbundledFile;

pub struct Unbundler {
    pub file_path: PathBuf,
    pub dds_mode: bool,
}

impl Unbundler {
    pub fn unbundle(&self) -> Result<Vec<UnbundledDirectory>, UnbundlerError> {
        let mut unbundled_dirs = vec![];

        if self.file_path.is_file() {
            if !Unbundler::has_valid_extension(&self.file_path)? {
                return Err(UnbundlerError::Extension(
                    format!("Attempted to unbundle a file which has an invalid extension. Do not unbundle files with extensions: .ini, .stream, or .data"
                )));
            }

            let file_path = String::from(self.file_path.to_str().ok_or(UnbundlerError::NotUTF8)?);
            let file_name = String::from(
                self.file_path
                    .file_name()
                    .ok_or(UnbundlerError::NoFileName)?
                    .to_str()
                    .ok_or(UnbundlerError::NotUTF8)?,
            );
            let unbundled = self.parse_bundle(&file_name, &file_path)?;
            unbundled_dirs.push(unbundled);
            return Ok(unbundled_dirs);
        }

        let read_dir = self.file_path.read_dir()?;

        for entry in read_dir {
            if !Unbundler::has_valid_extension(&entry.as_ref()?.path())? {
                print!(
                    "Skipping file with invalid extension: {:?}",
                    &entry.as_ref()?.path().to_str(),
                );
                continue;
            }

            if let Some(file_path) = entry.as_ref()?.path().to_str() {
                if let Some(file_name) = entry.as_ref()?.file_name().to_str() {
                    let unbundled =
                        self.parse_bundle(&String::from(file_name), &String::from(file_path))?;
                    unbundled_dirs.push(unbundled);
                }
            }
        }

        Ok(unbundled_dirs)
    }

    fn has_valid_extension(path: &PathBuf) -> Result<bool, UnbundlerError> {
        match path.extension() {
            Some(ext) => match ext.to_str().ok_or(UnbundlerError::NotUTF8)? {
                "stream" | "ini" | "data" => Ok(false),
                _ => Ok(true),
            },
            None => Ok(true),
        }
    }

    fn parse_bundle(
        &self,
        file_name: &String,
        file_path: &String,
    ) -> Result<UnbundledDirectory, UnbundlerError> {
        let file = fs::read(file_path)?;
        let mut compressed_stream = ByteStream::new(file);
        match self.read_unbundled_files(&mut compressed_stream) {
            Ok(files) => Ok(UnbundledDirectory::new(String::from(file_name), files)),
            Err(e) => Err(UnbundlerError::Inflater(format!(
                "Error inflating: {}\n{:?}",
                file_path, e
            ))),
        }
    }

    fn read_unbundled_files(
        &self,
        compressed_stream: &mut ByteStream,
    ) -> Result<Vec<UnbundledFile>, UnbundlerError> {
        let mut unbundled_files: Vec<UnbundledFile> = vec![];

        let mut inflated_stream = match self.inflate_stream(compressed_stream) {
            Ok(inflated_stream) => inflated_stream,
            Err(e) => return Err(e),
        };

        let file_count = inflated_stream.read_uint();
        let _checksum = inflated_stream.read(256);
        let _file_names_and_extensions = inflated_stream.read((16 * file_count) as usize);

        for _i in 0..file_count {
            let unbundled_file = self.read_unbundled_file(&mut inflated_stream);
            unbundled_files.push(unbundled_file);
        }

        Ok(unbundled_files)
    }

    fn inflate_stream(
        &self,
        compressed_stream: &mut ByteStream,
    ) -> Result<ByteStream, UnbundlerError> {
        let _header = compressed_stream.read_uint();
        let _size = compressed_stream.read_uint();
        let _reserved = compressed_stream.read_uint();

        let mut inflated: Vec<u8> = vec![];

        while compressed_stream.remaining_bytes() > 0 {
            match self.append_block(compressed_stream, &mut inflated) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        Ok(ByteStream::new(inflated))
    }

    fn read_unbundled_file(&self, inflated_stream: &mut ByteStream) -> UnbundledFile {
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
            extension: Extensions::lookup(extension, self.dds_mode),
            path,
            data,
        }
    }

    fn append_block(
        &self,
        compressed_stream: &mut ByteStream,
        buffer: &mut Vec<u8>,
    ) -> Result<(), UnbundlerError> {
        let len = compressed_stream.read_uint();

        if len == (1 << 16) {
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

    fn decompress_block(
        &self,
        compressed_stream: &mut ByteStream,
        len: usize,
    ) -> Result<Vec<u8>, UnbundlerError> {
        let mut decoder = ZlibDecoder::new(vec![]);

        match decoder.write_all(&compressed_stream.read(len as usize)) {
            Ok(_) => {}
            Err(_) => return Err(UnbundlerError::DecoderWriteAll),
        }

        match decoder.finish() {
            Ok(block) => Ok(block),
            Err(_) => Err(UnbundlerError::DecoderFinish),
        }
    }
}

#[derive(Debug)]
pub enum UnbundlerError {
    IOError(String),
    DecoderFinish,
    DecoderWriteAll,
    Inflater(String),
    Extension(String),
    NotUTF8,
    NoFileName,
}

impl From<&io::Error> for UnbundlerError {
    fn from(value: &io::Error) -> Self {
        UnbundlerError::IOError(format!("{}", value))
    }
}

impl From<io::Error> for UnbundlerError {
    fn from(value: io::Error) -> Self {
        UnbundlerError::IOError(format!("{}", value))
    }
}
