use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::io::{Read, Write};
use std::path;

const CONFIG_REPO: &'static str = "repo_path";
const CONFIG_EDITOR: &'static str = "editor_path";

pub enum ConfigFile {
    Repo,
    Editor,
}

pub struct FileHandler;

pub trait FileSystem {
    fn create_dir(&self, path: &str) -> io::Result<()>;
}

impl FileSystem for FileHandler {
    fn create_dir(&self, path: &str) -> io::Result<()> {
        fs::create_dir_all(&path)
    }
}

pub trait ConfigManagement {
    fn config_dir_create(&self) -> io::Result<String>;
    fn config_dir_exists(&self) -> bool;
    fn config_read(&self, file: ConfigFile) -> io::Result<String>;
    fn config_write(&self, file: ConfigFile, value: String) -> io::Result<()>;
}

pub trait FileManagement {
    fn file_exists(&self, path: &str) -> bool;
    fn file_rm(&self, file: ConfigFile) -> io::Result<()>;
}

impl ConfigManagement for FileHandler {
    fn config_dir_create(&self) -> io::Result<String> {
        fs::create_dir_all(config_dir_path()).expect("Cannot create directory");
        Ok(config_dir_path())
    }

    fn config_dir_exists(&self) -> bool {
        self.file_exists(&config_dir_path())
    }

    fn config_read(&self, file: ConfigFile) -> io::Result<String> {
        let config_file_path = config_path(file);
        let mut file = fs::File::open(&config_file_path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect(&format!("Unable to read file at: {}", config_file_path));
        if contents.ends_with("\n") {
            contents.pop().expect("File is empty");
        }

        Ok(contents)
    }

    fn config_write(&self, file: ConfigFile, value: String) -> io::Result<()> {
        let config_file_path = config_path(file);
        let path = path::Path::new(&config_file_path);

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

impl FileManagement for FileHandler {
    fn file_exists(&self, path: &str) -> bool {
        fs::metadata(path).is_ok()
    }

    fn file_rm(&self, file: ConfigFile) -> io::Result<()> {
        let config_file_path = config_path(file);

        if !self.file_exists(&config_file_path) {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                format!("Path does not exist: {}", config_file_path),
            ));
        }

        fs::remove_file(&config_file_path)?;
        Ok(())
    }
}

fn config_path(file: ConfigFile) -> String {
    let file_name = match file {
        ConfigFile::Repo => CONFIG_REPO.to_string(),
        ConfigFile::Editor => CONFIG_EDITOR.to_string(),
    };

    match env::home_dir() {
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
    match env::home_dir() {
        Some(home_dir) => format!("{}/{}", home_dir.display(), ".eureka"),
        None => panic!("Could not resolve your $HOME directory"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_handler::ConfigManagement;
    use file_handler::FileManagement;
    use file_handler::FileSystem;
    use std::fs;
    use std::io;

    struct MockFileSystem;
    struct MockFileHandler;

    impl FileSystem for MockFileSystem {
        fn create_dir(&self, _path: &str) -> io::Result<()> {
            fs::create_dir_all(&_path)
        }
    }

    impl ConfigManagement for MockFileHandler {
        fn config_dir_create(&self) -> io::Result<String> {
            fs::create_dir_all(config_dir_path()).expect("Cannot create directory");
            Ok(config_dir_path())
        }

        fn config_dir_exists(&self) -> bool {
            self.file_exists(&config_dir_path())
        }

        fn config_read(&self, _file: ConfigFile) -> io::Result<String> {
            let _config_file_path = config_path(_file);
            let mut file = fs::File::open(&_config_file_path)?;

            let mut _contents = String::new();
            file.read_to_string(&mut _contents)
                .expect(&format!("Unable to read file at: {}", _config_file_path));
            if _contents.ends_with("\n") {
                _contents.pop().expect("File is empty");
            }

            Ok(_contents)
        }

        fn config_write(&self, _file: ConfigFile, _value: String) -> io::Result<()> {
            let _config_file_path = config_path(_file);
            let _path = path::Path::new(&_config_file_path);

            let mut file = match fs::File::create(&_path) {
                Err(e) => panic!("Couldn't create {}: {}", _path.display(), e.description()),
                Ok(file) => file,
            };

            match file.write_all(_value.as_bytes()) {
                Err(e) => panic!("Couldn't write to {}: {}", _path.display(), e.description()),
                Ok(_) => Ok(()),
            }
        }
    }

    impl FileManagement for MockFileHandler {
        fn file_exists(&self, _path: &str) -> bool {
            fs::metadata(_path).is_ok()
        }

        fn file_rm(&self, _file: ConfigFile) -> io::Result<()> {
            let _config_file_path = config_path(_file);

            if !self.file_exists(&_config_file_path) {
                return Err(io::Error::new(
                    ErrorKind::NotFound,
                    format!("Path does not exist: {}", _config_file_path),
                ));
            }

            fs::remove_file(&_config_file_path)?;
            Ok(())
        }
    }
    // test for FileSystem methods
    #[test]
    fn create_dir() {
        let _fs = MockFileSystem {};
        let actual = _fs.create_dir("./testdir");
        println!("{:?}", actual);
        assert!(actual.is_ok());
    }

    // tests for FileHandler methods
    #[test]
    // technically, the two following tests also test the functionality of the file_exists function.. so a separate test for that is kind of redundant
    fn create_config_dir() {
        let _fh = MockFileHandler {};
        let dir = _fh.config_dir_create().unwrap();
        assert_eq!(_fh.file_exists(&dir), true);
    }

    #[test]
    fn check_config_dir_exists() {
        let _fh = MockFileHandler {};
        let dir = config_dir_path();
        assert_eq!(_fh.file_exists(&dir), true);
    }

    #[test]
    fn write_config_file() {
        let _fh = MockFileHandler {};
        let repo = String::from("~/idea");
        let write = _fh.config_write(ConfigFile::Repo, repo);
        assert!(write.is_ok());
    }

    #[test]
    fn read_config_file() {
        let _fh = MockFileHandler {};
        let repo = _fh.config_read(ConfigFile::Repo).unwrap();
        assert_eq!(repo, "~/idea");
    }

    #[test]
    fn delete_config_file() {
        let _fh = MockFileHandler {};
        let delete = _fh.file_rm(ConfigFile::Repo);
        assert!(delete.is_ok());
    }

}
