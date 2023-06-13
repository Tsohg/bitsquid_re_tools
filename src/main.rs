use bitsquid_unbundler::unbundler_controller::UnbundlerController;
use cla_input::Cla;
use file_writer::FileWriter;

extern crate bitsquid_unbundler;
extern crate luajit_decompiler;

mod cla_input;
mod file_writer;

fn main() {
    let clap = Cla::new();
    let unbundler = UnbundlerController::new(&clap);
    unbundler.unbundle(&mut FileWriter::new());
}
