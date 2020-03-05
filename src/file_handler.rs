extern crate dirs;

use self::dirs::home_dir;
use std::io::{ErrorKind, Read, Write};
use std::{fs, io, path};
use types::ConfigFile;

pub struct FileHandler;

pub trait FileManagement {
    fn create_dir(&self, path: &str) -> io::Result<()>;
    fn file_exists(&self, path: &str) -> bool;
    fn file_rm(&self, file: ConfigFile) -> io::Result<()>;
}

impl FileManagement for FileHandler {
    fn create_dir(&self, path: &str) -> io::Result<()> {
        fs::create_dir_all(&path)
    }

    fn file_exists(&self, path: &str) -> bool {
        fs::metadata(path).is_ok()
    }

    fn file_rm(&self, config: ConfigFile) -> io::Result<()> {
        let config_path = config_path_for(config);

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

pub trait ConfigManagement {
    fn config_dir_create(&self) -> io::Result<String>;
    fn config_dir_exists(&self) -> bool;
    fn config_read(&self, file: ConfigFile) -> io::Result<String>;
    fn config_write(&self, file: ConfigFile, value: &String) -> io::Result<()>;
}

impl ConfigManagement for FileHandler {
    fn config_dir_create(&self) -> io::Result<String> {
        fs::create_dir_all(config_dir_path()).expect("Cannot create directory");
        Ok(config_dir_path())
    }

    fn config_dir_exists(&self) -> bool {
        self.file_exists(&config_dir_path())
    }

    fn config_read(&self, config: ConfigFile) -> io::Result<String> {
        let config_path = config_path_for(config);
        if !self.file_exists(&config_path) {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                format!("File does not exist: {}", &config_path),
            ));
        }
        let mut file = fs::File::open(&config_path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect(&format!("Unable to read config at: {}", config_path));

        if contents.is_empty() {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                format!("File is empty: {}", &config_path),
            ));
        } else if contents.ends_with("\n") {
            contents
                .pop()
                .expect(&format!("File is empty: {}", &config_path));
        }

        Ok(contents)
    }

    fn config_write(&self, config: ConfigFile, value: &String) -> io::Result<()> {
        let config_path = &config_path_for(config);
        let path = path::Path::new(config_path);

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

fn config_path_for(config_type: ConfigFile) -> String {
    let file_name = match config_type {
        ConfigFile::Repo => ConfigFile::Repo.value(),
        ConfigFile::Editor => ConfigFile::Editor.value(),
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

fn config_dir_path() -> String {
    match home_dir() {
        Some(home_dir) => format!("{}/{}", home_dir.display(), ".eureka"),
        None => panic!("Could not resolve your $HOME directory"),
    }
}
