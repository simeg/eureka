use crate::dirs::home_dir;

use std::io::{ErrorKind, Read, Write};
use std::{fs, io, path};

#[derive(Debug, Eq, PartialEq)]
pub enum ConfigFile {
    Branch,
    Repo,
}

pub trait FileManagement {
    fn file_rm(&self, file: ConfigFile) -> io::Result<()>;
}

pub trait ConfigManagement {
    fn config_dir_create(&self) -> io::Result<String>;
    fn config_dir_exists(&self) -> bool;
    fn config_read(&self, file: ConfigFile) -> io::Result<String>;
    fn config_write(&self, file: ConfigFile, value: String) -> io::Result<()>;
}

pub struct FileHandler;

impl Default for FileHandler {
    fn default() -> Self {
        Self {}
    }
}

impl FileManagement for FileHandler {
    fn file_rm(&self, config: ConfigFile) -> io::Result<()> {
        let config_path = self.config_path_for(config);

        if fs::metadata(&config_path).is_err() {
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
        let dir_path = self.config_dir_path();
        fs::create_dir_all(&dir_path)
            .unwrap_or_else(|_| panic!("Cannot create directory: {}", &dir_path));
        Ok(dir_path)
    }

    fn config_dir_exists(&self) -> bool {
        fs::metadata(&self.config_dir_path()).is_ok()
    }

    fn config_read(&self, config: ConfigFile) -> io::Result<String> {
        let config_path = self.config_path_for(config);
        if fs::metadata(&config_path).is_err() {
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
            // These represents files so underscore is preferred
            ConfigFile::Repo => "branch",
            ConfigFile::Branch => "repo_path",
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
        let home = home_dir().expect("Could not resolve your $HOME directory");
        format!("{}/{}", home.display(), ".eureka")
    }
}
