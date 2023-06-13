use std::fs::{File, self};
use std::io::prelude::*;
use std::path::PathBuf;

use bitsquid_unbundler::unbundler_controller::OutputAdapter;
use re_core::unbundled_directory::UnbundledDirectory;
use re_core::unbundled_file::UnbundledFile;

pub struct FileWriter {
    count: usize,
    total: usize,
}
impl FileWriter {
    pub fn new() -> FileWriter {
        FileWriter { count: 1, total: 0 }
    }

    pub fn write_files(&mut self, output_dir: &PathBuf, unbundled_dir: &UnbundledDirectory) {
        for file in unbundled_dir.files.iter() {
            let mut path = output_dir.clone();
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

impl OutputAdapter for FileWriter {
    fn output(&mut self, output_dir: &PathBuf, unbundled: Vec<UnbundledDirectory>) {
        let mut assets = output_dir.clone();
        assets.push(r"assets\");

        match fs::create_dir(&assets) {  
            Ok(_) => (),
            Err(_) => (),
        }

        for dir in &unbundled {
            self.total += dir.files.len();
        }

        for dir in &unbundled {
            self.write_files(&assets, &dir);
        }
    }
}