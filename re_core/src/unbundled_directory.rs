use crate::unbundled_file::UnbundledFile;

pub struct UnbundledDirectory {
    pub dir_name: String,
    pub files: Vec<UnbundledFile>,
}

impl UnbundledDirectory {
    pub fn new(name: String, files: Vec<UnbundledFile>) -> UnbundledDirectory {
        UnbundledDirectory {
            dir_name: name,
            files: files,
        }
    }

    pub fn push(&mut self, file: UnbundledFile) {
        self.files.push(file);
    }
}