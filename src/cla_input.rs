use std::{path::PathBuf, ffi::OsString, str::FromStr};

use clap::{arg, command, value_parser, ArgMatches};

use bitsquid_unbundler::unbundler_controller::{InputAdapter, InputOptions};

pub struct Cla {
    matches: ArgMatches,
}

impl Cla {
    pub fn new() -> Cla {
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
            .arg(arg!(-d --dds ... "Unbundles texture files as dds files instead.")
                .required(false))        
            .get_matches();
        Cla { matches: matches }
    }
}

impl InputAdapter for Cla {
    fn input_options(&self) -> InputOptions {
        let input_path;
        match self.matches.get_one::<OsString>("input") {
            Some(path) => match PathBuf::from_str(path.to_str().unwrap()) {
                Ok(pathbuf) => input_path = Some(pathbuf),
                Err(_) => input_path = None,
            },
            None => input_path = None,
        }  
        InputOptions { input_path, dds_mode: self.matches.get_count("dds") > 0  }
    }

    fn output_dir(&self) -> Option<PathBuf> {
        match self.matches.get_one::<OsString>("output") {
            Some(path) => match PathBuf::from_str(path.to_str().unwrap()) {
                Ok(mut pathbuf) => { 
                    pathbuf.push("assets/");
                    Some(pathbuf) 
                },
                Err(_) => None,
            },
            None => None,
        }
    }
}

