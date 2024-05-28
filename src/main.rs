use bitsquid_unbundler::unbundler::Unbundler;
use command_line::CommandLine;
use file_writer::FileWriter;

use compiler_bootstrap::bootstrap::Bootstrapper;

extern crate bitsquid_unbundler;
// extern crate luajit_decompiler;
extern crate compiler_bootstrap;

mod command_line;
mod file_writer;

fn main() {
    let cmd = CommandLine::new();

    let tool = cmd
        .matches
        .get_one::<String>("tool")
        .expect("-t argument was not given.")
        .clone();

    match tool.as_str() {
        "bitsquid_unbundler" => {
            let unbundler: &mut Unbundler = &mut cmd.clone().into();
            let unbundled = unbundler.unbundle().unwrap();
            let file_writer: &mut FileWriter = &mut cmd.clone().into();

            for unbundled_dir in unbundled {
                file_writer.write_files(&unbundled_dir);
            }
        }
        "compiler_bootstrap" => {
            let bootstrapper: &Bootstrapper = &cmd.into();
            bootstrapper
                .compile()
                .expect("An IO error has occurred during compilation.");
        }
        "luajit_decompiler" => (), //soon^tm
        _ => panic!("Unknown tool (-t). Please see the supported tools with the --help command."),
    }
}
