use command_line::CommandLine;
use file_writer::FileWriter;

use bitsquid_unbundler::unbundler_controller::UnbundlerController;
use compiler_bootstrap::bootstrap::Bootstrapper;

extern crate bitsquid_unbundler;
// extern crate luajit_decompiler;
extern crate compiler_bootstrap;

mod command_line;
mod file_writer;

fn main() {
    let cmd = CommandLine::new();
    let bootstrapper: &Bootstrapper = &cmd.into();
    let unbundler: &UnbundlerController = &cmd.into();
    let file_writer: &FileWriter = &cmd.into();

    let tool = cmd.matches.get_one::<String>("tool").expect("-t argument was not given.").clone();

    match tool.as_str() {
        "bitsquid_unbundler" => {
            let unbundled = unbundler.unbundle();

            for unbundled_dir in unbundled {
                file_writer.write_files(&unbundled_dir);
            }
        }
        "compiler_bootstrap" => { 
            bootstrapper.compile().expect("An IO error has occurred during compilation."); 
        },
        "luajit_decompiler" => (), //soon^tm
        _ => panic!("Unknown tool (-t). Please see the supported tools with the --help command.")
    }
}