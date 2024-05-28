use std::path::PathBuf;

use bitsquid_unbundler::unbundler::Unbundler;
use clap::{arg, command, value_parser, ArgMatches};
use compiler_bootstrap::bootstrap::Bootstrapper;
use registry::{Hive, Security};

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

            .arg(arg!(-r --data_dir <DATA_DIR> "The data directory which contains the bundles.")
                .required(false).value_parser(value_parser!(String)))

            .arg(arg!(-d --dds ... "Unbundles texture files as dds files instead.")
                .required(false))

            .get_matches();
        CommandLine { matches }
    }

    //TODO: move this somewhere else. like a file for finding env vars.
    fn find_mww_bundles() -> String {
        let steam_dir = Hive::LocalMachine
            .open(r"SOFTWARE\WOW6432Node\Valve\Steam", Security::Read)
            .expect("Failed to read winreg for steam.\n");
        let data_win32_path = r"\steamapps\common\MagickaWizardWars\data_win32_bundled";
        //TODO: don't unwrap. there's an edge case where they don't have mww installed through steam or otherwise.
        let bundle_directory = String::from(format!("{}{}", steam_dir.value("InstallPath").unwrap().to_string(), data_win32_path));
        bundle_directory
    }
}

impl Into<Unbundler> for CommandLine {
    fn into(self) -> Unbundler {
        let dds_mode = self.matches.get_count("dds") > 0;
        let input_path;

        match self.matches.get_one::<String>("input") {
            Some(path) => input_path = String::from(path),
            None => input_path = CommandLine::find_mww_bundles(),
        }

        Unbundler::new(PathBuf::from(input_path), dds_mode)
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

impl Clone for CommandLine {
    fn clone(&self) -> Self {
        Self { matches: self.matches.clone() }
    }
}