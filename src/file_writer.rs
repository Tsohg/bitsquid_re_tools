use std::fs::{File, self};
use std::io::prelude::*;
use std::path::PathBuf;

use bitsquid_unbundler::unbundled_directory::UnbundledDirectory;
use bitsquid_unbundler::unbundled_file::UnbundledFile;

pub struct FileWriter {
    output_directory: PathBuf,
    count: usize,
    total: usize,
}

impl FileWriter {
    pub fn new(output_directory: PathBuf) -> FileWriter {
        FileWriter {
            output_directory,
            count: 1, 
            total: 0 ,
        }
    }

    pub fn write_files(&mut self, unbundled_dir: &UnbundledDirectory) {
        for file in unbundled_dir.files.iter() {
            let mut path = self.output_directory.clone();
            path.push(format!(r"{}\", unbundled_dir.dir_name));
            
            match fs::create_dir(&path) {  
                Ok(_) => (),
                Err(_) => (),
            }

            path.push(format!("{:#x}.{}", file.path, file.extension));
            self.write_file(&path, file);
        }
    }

    pub fn write_file(&mut self, file_path: &PathBuf, unbundled_file: &UnbundledFile) {
        let mut file = File::create(file_path).unwrap();
        file.write_all(&unbundled_file.data).unwrap();
        println!("[{}/{}] {}", self.count, self.total, file_path.file_name().unwrap().to_str().unwrap());
        self.count += 1;
    }
}