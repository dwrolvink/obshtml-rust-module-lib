use std::io;
use crate::lib::paths::{RelativePosixPath, AbsolutePosixPath};
use crate::lib::file;
use crate::module::baseclass::ObsidianModule;

pub struct Modfile {
    module_data_folder: AbsolutePosixPath,
    file_path: RelativePosixPath,
}

impl Modfile  {
    pub fn new(obsmod: &ObsidianModule, file_path: &str) -> Modfile {
        Modfile {
            module_data_folder: obsmod.module_data_folder.clone(),
            file_path: RelativePosixPath(file_path.to_string()),
        }
    }

    pub fn get_abs_file_path(&self) -> String {
        let mut abs_file_path = self.module_data_folder.to_string();
        abs_file_path.push_str("/");
        abs_file_path.push_str(&self.file_path.to_string());
        return abs_file_path;
    }

    pub fn read(&self) -> Option<String> {
        let abs_file_path = self.get_abs_file_path();
        file::read(&abs_file_path)
    }

    pub fn write(&self, contents: &str) -> io::Result<()> {
        let abs_file_path = self.get_abs_file_path();
        file::write(&abs_file_path, contents)
    }
}