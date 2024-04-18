use std::{process::{Command, Output}, io};

//bitsquid_win32_dev.exe -compile -source-dir MWW-master -data-dir MWW-dump -bundle-dir MWW-bundle
pub struct Bootstrapper {
    pub compiler_path: String,
    pub src_dir: String,
    pub data_dir: String,
    pub bundle_dir: String,
}

impl Bootstrapper {
    pub fn compile(&self) -> io::Result<Output> {
        let mut cmd = Command::new(&self.compiler_path);
        cmd.args([
            "-compile", 
            "-source-dir", &self.src_dir, 
            "-data-dir", &self.data_dir, 
            "-bundle-dir", &self.bundle_dir]);

        println!("Executing: {:?}", cmd);
        cmd.output()
    }
}