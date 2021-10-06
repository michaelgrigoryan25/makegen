#[allow(unused_imports)]
use crate::constants::{
    ERROR_ACCESS_DENIED, ERROR_CONVERTING_PATHBUF, ERROR_OPENING_FILE, ERROR_WRITING_FILE,
    GENERATED_BY_MAKEGEN_COMMENT,
};
use crate::utils::dir_path_as_string;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug, Default, Clone)]
pub struct FileSystem {
    base_path: String,
}

pub trait FileSystemActions {
    fn get_base_path(&mut self) -> String;
    fn set_base_path(&mut self, path: String);
    fn write_buffer(&mut self, data: &mut String);
}

impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem {
            base_path: dir_path_as_string(),
        }
    }
}

impl FileSystemActions for FileSystem {
    fn get_base_path(&mut self) -> String {
        self.base_path.to_owned()
    }

    fn set_base_path(&mut self, path: String) {
        self.base_path = path;
    }

    fn write_buffer(&mut self, data: &mut String) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(&self.get_base_path())
            .expect(ERROR_OPENING_FILE);

        file.write_all(GENERATED_BY_MAKEGEN_COMMENT.as_bytes())
            .expect(ERROR_WRITING_FILE);
        file.write_all('\n'.to_string().as_bytes())
            .expect(ERROR_WRITING_FILE);
        file.write_all(data.as_bytes()).expect(ERROR_WRITING_FILE);
    }
}

#[cfg(test)]
mod filesystem_tests {
    use super::{FileSystem, FileSystemActions};

    #[test]
    fn test_write_buffer() {
        let mut fs = FileSystem::new();
        let mut data = "hello world".to_string();
        fs.write_buffer(&mut data);
    }
}
