extern crate dirs;

use self::dirs::home_dir;
use std::error::Error;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::io::{Read, Write};
use std::path;

pub enum Config {
    Repo,
    Editor,
}

impl Config {
    fn value(&self) -> &str {
        match *self {
            Config::Repo => "repo_path",
            Config::Editor => "editor_path",
        }
    }
}

pub struct FileHandler;

pub trait FileManagement {
    fn create_dir(&self, path: &str) -> io::Result<()>;
    fn file_exists(&self, path: &str) -> bool;
    fn file_rm(&self, file: Config) -> io::Result<()>;
}

impl FileManagement for FileHandler {
    fn create_dir(&self, path: &str) -> io::Result<()> {
        fs::create_dir_all(&path)
    }

    fn file_exists(&self, path: &str) -> bool {
        fs::metadata(path).is_ok()
    }

    fn file_rm(&self, config: Config) -> io::Result<()> {
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
    fn config_read(&self, file: Config) -> io::Result<String>;
    fn config_write(&self, file: Config, value: String) -> io::Result<()>;
}

impl ConfigManagement for FileHandler {
    fn config_dir_create(&self) -> io::Result<String> {
        fs::create_dir_all(config_dir_path()).expect("Cannot create directory");
        Ok(config_dir_path())
    }

    fn config_dir_exists(&self) -> bool {
        self.file_exists(&config_dir_path())
    }

    fn config_read(&self, config: Config) -> io::Result<String> {
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

    fn config_write(&self, config: Config, value: String) -> io::Result<()> {
        let config_path = &config_path_for(config);
        let path = path::Path::new(config_path);

        let mut file = match fs::File::create(&path) {
            Err(e) => panic!("Couldn't create {}: {}", path.display(), e.description()),
            Ok(file) => file,
        };

        match file.write_all(value.as_bytes()) {
            Err(e) => panic!("Couldn't write to {}: {}", path.display(), e.description()),
            Ok(_) => Ok(()),
        }
    }
}

fn config_path_for(file: Config) -> String {
    let file_name = match file {
        Config::Repo => Config::Repo.value(),
        Config::Editor => Config::Editor.value(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use file_handler::ConfigManagement;
    use file_handler::FileManagement;
    use std::io;

    struct MockFileSystem;
    struct MockFileHandler;

    impl ConfigManagement for MockFileHandler {
        fn config_dir_create(&self) -> io::Result<String> {
            Ok(String::from("irrelevant"))
        }

        fn config_dir_exists(&self) -> bool {
            self.file_exists(&String::from("irrelevant"))
        }

        fn config_read(&self, _file: Config) -> io::Result<String> {
            Ok(String::from("irrelevant"))
        }

        fn config_write(&self, _file: Config, _value: String) -> io::Result<()> {
            Ok(())
        }
    }

    impl FileManagement for MockFileHandler {
        fn create_dir(&self, _path: &str) -> io::Result<()> {
            Ok(())
        }

        fn file_exists(&self, _path: &str) -> bool {
            true
        }

        fn file_rm(&self, _file: Config) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn tests_work() {
        assert!(true, true);
    }
}
