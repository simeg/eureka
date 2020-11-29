use crate::dirs::home_dir;

use std::io::{ErrorKind, Read, Write};
use std::{fs, io, path};

#[derive(Debug, Eq, PartialEq)]
pub enum ConfigType {
    Branch,
    Repo,
}

pub trait ConfigManagement {
    fn config_dir_create(&self) -> io::Result<()>;
    fn config_dir_exists(&self) -> bool;
    fn config_read(&self, config_type: ConfigType) -> io::Result<String>;
    fn config_write(&self, config_type: ConfigType, value: String) -> io::Result<()>;
    fn config_rm(&self, config_type: ConfigType) -> io::Result<()>;
}

pub struct ConfigManager;

impl Default for ConfigManager {
    fn default() -> Self {
        Self {}
    }
}

impl ConfigManagement for ConfigManager {
    fn config_dir_create(&self) -> io::Result<()> {
        self.config_dir_path().and_then(fs::create_dir_all)
    }

    fn config_dir_exists(&self) -> bool {
        self.config_dir_path().and_then(fs::metadata).is_ok()
    }

    fn config_read(&self, config_type: ConfigType) -> io::Result<String> {
        let config_path = self.config_path_for(config_type)?;
        // Make sure file exists
        fs::metadata(&config_path)?;

        let mut file = fs::File::open(&config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        if contents.is_empty() {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                format!("File is empty: {}", &config_path),
            ));
        } else if contents.ends_with('\n') {
            contents.pop().ok_or_else(|| {
                io::Error::new(ErrorKind::Other, "Unable to remove last char from file")
            })?;
        }

        Ok(contents)
    }

    fn config_write(&self, config_type: ConfigType, value: String) -> io::Result<()> {
        let config_path = self.config_path_for(config_type)?;

        // Create file if it doesn't exist, otherwise get it
        let mut file = fs::File::create(&path::Path::new(config_path.as_str()))?;
        file.write_all(value.as_bytes())
    }

    fn config_rm(&self, config_type: ConfigType) -> io::Result<()> {
        let config_path = self.config_path_for(config_type)?;
        // Make sure file exists
        fs::metadata(&config_path)?;
        fs::remove_file(&config_path)
    }
}

impl ConfigManager {
    fn config_path_for(&self, config_type: ConfigType) -> io::Result<String> {
        let file_name = match config_type {
            // These represents files so underscore is preferred
            ConfigType::Repo => "repo_path",
            ConfigType::Branch => "branch",
        };

        self.config_dir_path()
            .map(|path| format!("{}/{}", path, file_name))
    }

    fn config_dir_path(&self) -> io::Result<String> {
        home_dir()
            .map(|home| format!("{}/{}", home.display(), ".eureka"))
            .ok_or_else(|| {
                io::Error::new(
                    ErrorKind::NotFound,
                    "Could not resolve your $HOME directory",
                )
            })
    }
}
