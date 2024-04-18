use std::path::PathBuf;

use crate::{unbundled_directory::UnbundledDirectory, unbundled_file::UnbundledFile, unbundler::Unbundler};

pub trait UnbundlerArgs {
    fn input_path(&self) -> PathBuf;
    fn dds_mode(&self) -> bool;
}

pub struct UnbundlerController {
    input_path: PathBuf,
    dds_mode: bool,
}

impl UnbundlerController {
    pub fn new(args: &impl UnbundlerArgs) -> UnbundlerController {
        UnbundlerController {
            input_path: args.input_path(),
            dds_mode: args.dds_mode(),
        }
    }

    pub fn unbundle(&self) -> Vec<UnbundledDirectory>{
        let mut unbundled = vec![];

        if self.input_path.is_dir() {
            let read_dir = self.input_path.read_dir().unwrap();
            for entry in read_dir {
                unbundled.push(self.unbundle_directory(&entry.unwrap().path()))
            }
        } else {
            unbundled.push(self.unbundle_directory(&self.input_path));
        }

        unbundled
    }

    fn unbundle_directory(&self, path: &PathBuf) -> UnbundledDirectory {
        let files = self.unbundle_file(path);
        UnbundledDirectory::new(String::from(path.file_name().unwrap().to_str().unwrap()), files)
    }

    fn unbundle_file(&self, path: &PathBuf) -> Vec<UnbundledFile> {
        if !UnbundlerController::has_valid_extension(path) { return vec![]; }

        let input_path = path.to_str().unwrap();
        let mut unbundler = Unbundler::new(input_path).unwrap(); //TODO: handle io::error

        match unbundler.unbundle_file(self.dds_mode) {
            Ok(unbundled_files) => unbundled_files,
            Err(e) => { 
                print!("Encountered an error unbundling file: {}\n{:?}", input_path, e);
                return vec![];
            },
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
}