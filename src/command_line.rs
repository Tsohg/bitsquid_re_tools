use std::{path::PathBuf, str::FromStr};

use clap::{arg, command, value_parser, ArgMatches};
use compiler_bootstrap::bootstrap::Bootstrapper;
use registry::{Hive, Security};

use bitsquid_unbundler::unbundler_controller::UnbundlerArgs;

use crate::file_writer::FileWriter;

pub struct CommandLine {
    pub matches: ArgMatches,
}

impl CommandLine {
    pub fn new() -> CommandLine {
        let matches = command!()
            .name("Bitsquid Reverse Engineering Tools")
            .version("1.0.0")
            .author("Alias")
            .about("A toolchain for developers reverse engineering the bitsquid engine.")

            .arg(arg!(-t --tool <TOOL> "Currently supported tools: -t bitsquid_unbundler\n-t luajit_decompiler\ncompiler_bootstrap\n")
                .required(true).value_parser(value_parser!(String)))

            .arg(arg!(-i --input <INPUT> "Input may be a path to a file or directory.")
                .required(false).value_parser(value_parser!(String)))

            .arg(arg!(-o --output <OUTPUT> "Output may be a path to a file or a directory.")
                .required(false).value_parser(value_parser!(String)))

            .arg(arg!(-c --compiler <COMPILER> "An executable bitsquid compiler. Some bitsquid game exes allow for the --compile arg to be used.")
                .required(false).value_parser(value_parser!(String)))

            .arg(arg!(-r --data_dir <DATA_DIR> "The -data-dir used by the compiler bootstrap tool.")
                .required(false).value_parser(value_parser!(String)))

            .arg(arg!(-d --dds ... "Unbundles texture files as dds files instead.")
                .required(false))

            .get_matches();
        CommandLine { matches }
    }

    //TODO: move this somewhere else. like a file for finding env vars. also call it data_dir not bundles
    fn find_mww_bundles() -> PathBuf {
        let steam_dir = Hive::LocalMachine
            .open(r"SOFTWARE\WOW6432Node\Valve\Steam", Security::Read)
            .expect("Failed to read winreg for steam.\n");
        let data_win32_path = r"\steamapps\common\MagickaWizardWars\data_win32_bundled";
        let bundle_directory = String::from(format!("{}{}", steam_dir.value("InstallPath").unwrap().to_string(), data_win32_path));
        PathBuf::from_str(&bundle_directory).unwrap() //safe for utf8 only.
    }
}

impl UnbundlerArgs for CommandLine {
    fn input_path(&self) -> PathBuf {
        let input_path;
        match self.matches.get_one::<String>("input") {
            Some(path) => match PathBuf::from_str(path) {
                Ok(pathbuf) => input_path = pathbuf,
                Err(_) => input_path = CommandLine::find_mww_bundles(),
            },
            None => input_path = CommandLine::find_mww_bundles(),
        }  
        input_path
    }

    fn dds_mode(&self) -> bool {
        self.matches.get_count("dds") > 0
    }
}

impl Into<UnbundlerArgs> for CommandLine {
    fn into(self) -> UnbundlerArgs {
        
    }
}

impl Into<FileWriter> for CommandLine {
    fn into(self) -> FileWriter {
        let output_dir = self.matches.get_one::<String>("output").expect("todo: Into<FileWriter>");
        FileWriter::new(PathBuf::from(output_dir))
    }
}

impl Into<Bootstrapper> for CommandLine {
    fn into(self) -> Bootstrapper {
        let compiler_path = self.matches.get_one::<String>("compiler")
            .expect("When using the compiler bootstrap tool, you must supply an absolute path to the -c (-compiler) argument.")
            .to_string();

        let src_dir = self.matches.get_one::<String>("input")
            .expect("The input -i argument for the compiler bootstrap tool is required for the -source-dir argument.")
            .to_string();

        let data_dir = self.matches.get_one::<String>("data_dir")
            .expect("The data directory -r argument for the compiler bootstrap tool is required for the -data-dir argument.")
            .to_string();

        let bundle_dir = self.matches.get_one::<String>("output")
            .expect("The output -o argument for the compiler bootstrap tool is required for the -bundle-dir argument.")
            .to_string();

        Bootstrapper {
            compiler_path,
            src_dir,
            data_dir,
            bundle_dir,
        }
    }
}