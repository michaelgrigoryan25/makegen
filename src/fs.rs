use std::fs::File;
use std::io::{Result, Write};

use crate::constants::ERROR_WRITING_FILE;

#[derive(Debug, Default, Clone)]
pub struct FS {
    base_path: String,
}

pub trait FsActions {
    fn get_base_path(&mut self) -> String;
    fn set_base_path(&mut self, path: String) -> ();
    fn write_buffer(&mut self, path: String, data: String) -> Result<usize>;
}

impl FS {
    pub fn new() -> FS {
        FS {
            base_path: "./".to_string(),
        }
    }
}

impl FsActions for FS {
    fn set_base_path(&mut self, path: String) -> () {
        self.base_path = path;
    }

    fn write_buffer(&mut self, path: String, data: String) -> Result<usize> {
        File::create(&path)
            .expect(&ERROR_WRITING_FILE)
            .write(data.as_bytes())
    }

    fn get_base_path(&mut self) -> String {
        self.base_path.to_owned()
    }
}
