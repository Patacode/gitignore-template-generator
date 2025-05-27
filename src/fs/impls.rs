use std::{fs, io::Error};

use super::FileSystemHandler;

pub struct DirectoryHandler<'a> {
    pub directory_path: &'a str,
}

impl<'a> DirectoryHandler<'a> {
    pub fn new(directory_path: &'a str) -> Self {
        Self { directory_path }
    }
}

impl FileSystemHandler for DirectoryHandler<'_> {
    fn fetch_content(&self, file_name: &str) -> Result<String, Error> {
        fs::read_to_string(format!("{}/{file_name}", self.directory_path))
    }

    fn list_files(&self) -> Result<Vec<String>, Error> {
        let mut result: Vec<String> = Vec::new();

        for entry in fs::read_dir(self.directory_path)? {
            let entry_path = entry?.path();

            if entry_path.is_file() {
                if let Some(file_stem) = entry_path.file_stem() {
                    result.push(file_stem.to_string_lossy().to_string());
                }
            }
        }

        Ok(result)
    }
}
