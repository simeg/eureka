use crate::dirs::home_dir;
use crate::types::ConfigFile;

use std::io::{ErrorKind, Read, Write};
use std::{fs, io, path};

pub trait FileManagement {
    fn dir_create(&self, path: &str) -> io::Result<()>;
    fn file_exists(&self, path: &str) -> bool;
    fn file_rm(&self, file: ConfigFile) -> io::Result<()>;
}

pub trait ConfigManagement {
    fn config_dir_create(&self) -> io::Result<String>;
    fn config_dir_exists(&self) -> bool;
    fn config_read(&self, file: ConfigFile) -> io::Result<String>;
    fn config_write(&self, file: ConfigFile, value: String) -> io::Result<()>;
}

pub struct FileHandler;

impl FileManagement for FileHandler {
    fn dir_create(&self, path: &str) -> io::Result<()> {
        fs::create_dir_all(&path)
    }

    fn file_exists(&self, path: &str) -> bool {
        fs::metadata(path).is_ok()
    }

    fn file_rm(&self, config: ConfigFile) -> io::Result<()> {
        let config_path = self.config_path_for(config);

        if !self.file_exists(&config_path) {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                format!("Path does not exist: {}", config_path),
            ));
        }

        fs::remove_file(&config_path)?;
        Ok(())
    }
}

impl ConfigManagement for FileHandler {
    fn config_dir_create(&self) -> io::Result<String> {
        fs::create_dir_all(self.config_dir_path()).expect("Cannot create directory");
        Ok(self.config_dir_path())
    }

    fn config_dir_exists(&self) -> bool {
        self.file_exists(&self.config_dir_path())
    }

    fn config_read(&self, config: ConfigFile) -> io::Result<String> {
        let config_path = self.config_path_for(config);
        if !self.file_exists(&config_path) {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                format!("File does not exist: {}", &config_path),
            ));
        }
        let mut file = fs::File::open(&config_path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .unwrap_or_else(|_| panic!("Unable to read config at: {}", config_path));

        if contents.is_empty() {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                format!("File is empty: {}", &config_path),
            ));
        } else if contents.ends_with('\n') {
            contents
                .pop()
                .unwrap_or_else(|| panic!("File is empty: {}", &config_path));
        }

        Ok(contents)
    }

    fn config_write(&self, config: ConfigFile, value: String) -> io::Result<()> {
        let config_path = &self.config_path_for(config);
        let path = path::Path::new(config_path);

        // Create file if it doesn't exist
        let mut file = match fs::File::create(&path) {
            Err(e) => panic!("Couldn't create {}: {}", path.display(), e.to_string()),
            Ok(file) => file,
        };

        match file.write_all(value.as_bytes()) {
            Err(e) => panic!("Couldn't write to {}: {}", path.display(), e.to_string()),
            Ok(_) => Ok(()),
        }
    }
}

impl FileHandler {
    fn config_path_for(&self, config_type: ConfigFile) -> String {
        let file_name = match config_type {
            ConfigFile::Repo => ConfigFile::Repo.value(),
            ConfigFile::Branch => ConfigFile::Branch.value(),
        };

        match home_dir() {
            Some(location) => format!(
                "{home}/{eureka}/{file_name}",
                home = location.display(),
                eureka = ".eureka",
                file_name = file_name
            ),
            None => panic!("Could not resolve your $HOME directory"),
        }
    }

    fn config_dir_path(&self) -> String {
        match home_dir() {
            Some(home_dir) => format!("{}/{}", home_dir.display(), ".eureka"),
            None => panic!("Could not resolve your $HOME directory"),
        }
    }
}
