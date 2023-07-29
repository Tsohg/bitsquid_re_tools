use std::{path::PathBuf, ffi::OsString, str::FromStr};

use registry::{Hive, Security};
use re_core::{unbundled_file::UnbundledFile, unbundled_directory::UnbundledDirectory};

use crate::unbundler::Unbundler;

pub trait InputAdapter {
    fn input_options(&self) -> InputOptions;
    fn output_dir(&self) -> Option<PathBuf>;
}

pub struct InputOptions {
    pub input_path: Option<PathBuf>,
    pub dds_mode: bool,
}

pub trait OutputAdapter {
    fn output(&mut self, output_dir: &PathBuf, unbundled: Vec<UnbundledDirectory>);
}

pub struct UnbundlerController {
    input_dir: PathBuf,
    output_dir: PathBuf,
    dds_mode: bool,
}

impl UnbundlerController {
    pub fn new(adapter: &impl InputAdapter) -> UnbundlerController {
        let input_dir;
        let options = adapter.input_options();
        match options.input_path {
            Some(path) => input_dir = path,
            None => input_dir = UnbundlerController::find_mww_bundles(),
        }

        let output_dir;
        match adapter.output_dir() {
            Some(path) => output_dir = path,
            None => output_dir = std::env::current_dir().expect("The current pwd does not exist or has insufficient permissions."),
        }

        UnbundlerController {
            input_dir,
            output_dir,
            dds_mode: options.dds_mode
        }
    }

    fn find_mww_bundles() -> PathBuf {
        let steam_dir = Hive::LocalMachine
            .open(r"SOFTWARE\WOW6432Node\Valve\Steam", Security::Read)
            .expect("Failed to read winreg for steam.\n");
        let data_win32_path = r"\steamapps\common\MagickaWizardWars\data_win32_bundled";
        let bundle_directory = OsString::from(format!("{}{}", steam_dir.value("InstallPath").unwrap().to_string(), data_win32_path));
        PathBuf::from_str(bundle_directory.to_str().unwrap()).unwrap() //safe for utf8 only.
    }

    pub fn unbundle(&self, adapter: &mut impl OutputAdapter){
        let mut unbundled = vec![];

        if self.input_dir.is_dir() {
            let read_dir = self.input_dir.read_dir().unwrap();
            for entry in read_dir {
                unbundled.push(self.unbundle_directory(&entry.unwrap().path()))
            }
        } else {
            unbundled.push(self.unbundle_directory(&self.input_dir));
        }

        adapter.output(&self.output_dir, unbundled);
    }

    fn unbundle_directory(&self, path: &PathBuf) -> UnbundledDirectory {
        let files = self.unbundle_file(path);
        UnbundledDirectory::new(String::from(path.file_name().unwrap().to_str().unwrap()), files)
    }

    fn unbundle_file(&self, path: &PathBuf) -> Vec<UnbundledFile> {
        if !UnbundlerController::has_valid_extension(path) { return vec![]; }

        let input_path = path.to_str().unwrap();
        let mut unbundler = Unbundler::new(input_path, self.dds_mode).unwrap(); //TODO: handle io::error

        match unbundler.unbundle_file() {
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