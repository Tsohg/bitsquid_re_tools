use std::{path::PathBuf, ffi::OsString, str::FromStr};

use bitsquid_unbundler::unbundler_controller::UnbundlerAdapter;
use clap::{arg, command, value_parser, ArgMatches};


pub struct ClaInput {
    matches: ArgMatches,
}

impl ClaInput {
    pub fn new() -> ClaInput {
        let matches = command!()
            .name("Bitsquid Reverse Engineering Tools")
            .version("1.0.0")
            .author("Alias")
            .about("A toolchain specifically designed for developers working with reverse engineered bitsquid assets.")
            .arg(arg!(-t --tool <TOOL> "Currently supported tools: -t bitsquid_unbundler\n-t luajit_decompiler\n")
                .required(true).value_parser(value_parser!(OsString)))
            .arg(arg!(-i --input <INPUT> "Input may be a path to a file or directory.")
                .required(false).value_parser(value_parser!(OsString)))
            .arg(arg!(-o --output <OUTPUT> "Output may be a path to a file or a directory.")
                .required(false).value_parser(value_parser!(OsString)))
            .get_matches();
        ClaInput { matches: matches }
    }
}

impl UnbundlerAdapter for ClaInput {
    fn input(&self) -> Option<PathBuf> {
        match self.matches.get_one::<OsString>("input") {
            Some(path) => match PathBuf::from_str(path.to_str().unwrap()) {
                Ok(pathbuf) => return Some(pathbuf),
                Err(_) => None,
            },
            None => None,
        }  
    }

    fn output(&self) -> Option<PathBuf> {
        match self.matches.get_one::<OsString>("output") {
            Some(path) => match PathBuf::from_str(path.to_str().unwrap()) {
                Ok(pathbuf) => Some(pathbuf),
                Err(_) => None,
            },
            None => None,
        }
    }
}

