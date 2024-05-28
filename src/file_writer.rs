use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;

use bitsquid_unbundler::unbundled_directory::UnbundledDirectory;
use bitsquid_unbundler::unbundled_file::UnbundledFile;

pub struct FileWriter {
    output_directory: PathBuf,
    count: usize,
}

impl FileWriter {
    pub fn new(output_directory: PathBuf) -> FileWriter {
        FileWriter {
            output_directory,
            count: 1,
        }
    }

    pub fn write_files(&mut self, unbundled_dir: &UnbundledDirectory, total: usize) {
        for file in unbundled_dir.files.iter() {
            let mut path = self.output_directory.clone();
            path.push(format!(r"{}\", unbundled_dir.dir_name));

            match fs::create_dir(&path) {
                Ok(_) => (),
                Err(_) => (),
            }

            path.push(format!("{:#x}.{}", file.path, file.extension));
            self.write_file(&path, file, total);
        }
    }

    fn write_file(&mut self, file_path: &PathBuf, unbundled_file: &UnbundledFile, total: usize) {
        match File::create(file_path) {
            Ok(mut file) => {
                file.write_all(&unbundled_file.data).unwrap();
                println!(
                    "[{}/{}] {}",
                    self.count,
                    total,
                    file_path.file_name().unwrap().to_str().unwrap()
                );
                self.count += 1;
            }
            Err(e) => {
                println!("IO Error: {}", e);
            }
        }
    }
}
