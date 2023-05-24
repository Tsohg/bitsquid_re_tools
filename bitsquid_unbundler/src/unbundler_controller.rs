use std::{path::PathBuf, ffi::OsString, str::FromStr};

use registry::{Hive, Security};

use crate::unbundled_file::UnbundledFile;


pub trait UnbundlerAdapter {
    fn input(&self) -> Option<PathBuf>;
    fn output(&self) -> Option<PathBuf>;
}

pub struct UnbundlerController {
    input: PathBuf,
    output: PathBuf,
}

impl UnbundlerController {
    pub fn new(adapter: &impl UnbundlerAdapter) -> UnbundlerController {
        let input;
        match adapter.input() {
            Some(path) => input = path,
            None => input = UnbundlerController::find_mww_bundles(),
        }
        
        let output;
        match adapter.output() {
            Some(path) => output = path,
            None => output = std::env::current_dir().expect("The current pwd does not exist or has insufficient permissions."),
        }

        UnbundlerController {
            input,
            output
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

    pub fn unbundle() {

    }

    fn unbundle_files() {

    }

    // fn unbundle_file(&self, path: &PathBuf) -> Option<UnbundledFile> {

    // }
}

        // match path.extension() {
        //     Some(ext) => match ext.to_str().unwrap() {
        //         "stream" => return,
        //         "ini" => return,
        //         "data" => return,
        //         _ => (),
        //     },
        //     None => (),
        // }

//----------------------\\

// fn read_input_dir_from_registry() -> ReadDir {
//     let error = "Could not find the Steam directory to locate the Magicka: Wizard Wars data_win32_bundled directory";
//     

//     
//     

//     fs::read_dir(bundle_directory).expect(error)
// }

// fn unbundle_files(bundle_dir: ReadDir, write_dir: PathBuf) {
//     for bundle_path_result in bundle_dir {
//         match bundle_path_result  {
//             Ok(bundle_path) => try_unbundle_file(bundle_path, &write_dir),
//             Err(e) => println!("An unexpected error occurred when reading a bundle file. Error: {}\nSkipping file.", e),
//         }
//     }
// }

// fn try_unbundle_file(bundle_path: DirEntry, write_dir: &PathBuf) {
//     let bundle_pathbuf = bundle_path.path();

//     match bundle_pathbuf.extension() {
//         Some(ext) => match ext.to_str().unwrap() {
//             "stream" => return,
//             "ini" => return,
//             "data" => return,
//             _ => (),
//         },
//         None => (),
//     }

//     //This should be a safe unwrap since error handling has already been done for bundle_path.
//     let path = bundle_pathbuf
//         .as_os_str()
//         .to_str()
//         .unwrap();

//     match unbundle_file(path) {
//         Ok(files) => {
//             let mut out_path = write_dir.clone();
//             out_path.push(bundle_path.file_name());
//             fs::create_dir(&out_path).unwrap();

//             //This should be a safe unwrap since error handling is already done for the output directory.
//             FileWriter::write_files(out_path.to_str().unwrap(), files);
//         },
//         Err(e) => println!("An error occurred while attempting to unbundle {}\n Error: {:?}", path, e),
//     }
// }

// fn unbundle_file(path: &str) -> Result<Vec<UnbundledFile>, UnbundlerError> {
//     match Unbundler::new(path) {
//         Ok(mut unbundler) => unbundler.unbundle_file(),
//         Err(_) => Err(UnbundlerError::IOError),
//     }
// }