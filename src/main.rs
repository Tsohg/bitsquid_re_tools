use bitsquid_unbundler::unbundler_controller::UnbundlerController;
use cla_input::ClaInput;

extern crate bitsquid_unbundler;
extern crate luajit_decompiler;

mod cla_input;

fn main() {
    let clap = ClaInput::new();
    let unbundler = UnbundlerController::new(&clap);
}
