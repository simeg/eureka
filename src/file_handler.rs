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

pub trait MyTrait {
    fn config_name(&self, file: ConfigFile) -> String;
    fn config_path(&self, file_name: String) -> String;
    fn config_location(&self) -> String;
    fn read_from_config(&self, file: ConfigFile) -> io::Result<String>;
    fn write_to_config(&self, file: ConfigFile, value: String) -> io::Result<()>;
    fn path_exists(&self, path: &str) -> bool;
    fn rm_file(&self, file: ConfigFile) -> io::Result<()>;
}

impl MyTrait for FileHandler {
    fn config_name(&self, file: ConfigFile) -> String {
        match file {
            ConfigFile::Repo => self.config_path(CONFIG_REPO.to_string()),
            ConfigFile::Editor => self.config_path(CONFIG_EDITOR.to_string()),
        }
    }

    fn config_path(&self, file_name: String) -> String {
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

    fn config_location(&self) -> String {
        match env::home_dir() {
            Some(location) => format!("{}/{}", location.display(), ".eureka"),
            None => panic!("Could not resolve your $HOME directory"),
        }
    }

    fn read_from_config(&self, file: ConfigFile) -> io::Result<String> {
        let file_name = self.config_name(file);
        let mut file = fs::File::open(&file_name)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect(&format!("Unable to read file at: {}", file_name));
        if contents.ends_with("\n") {
            contents.pop().expect("File is empty");
        }

        Ok(contents)
    }

    fn write_to_config(&self, file: ConfigFile, value: String) -> io::Result<()> {
        let file_name: String = self.config_name(file);
        let path = path::Path::new(&file_name);

        let mut file = match fs::File::create(&path) {
            Err(e) => panic!("Couldn't create {}: {}", path.display(), e.description()),
            Ok(file) => file,
        };

        match file.write_all(value.as_bytes()) {
            Err(e) => panic!("Couldn't write to {}: {}", path.display(), e.description()),
            Ok(_) => Ok(()),
        }
    }

    fn path_exists(&self, path: &str) -> bool {
        fs::metadata(path).is_ok()
    }

    fn rm_file(&self, file: ConfigFile) -> io::Result<()> {
        let path: String = self.config_path(self.config_name(file));
        if self.path_exists(&path) {
            fs::remove_file(&path)?;
            Ok(())
        } else {
            let invalid_path = io::Error::new(
                ErrorKind::NotFound,
                format!("Path does not exist: {}", path),
            );
            Err(invalid_path)
        }
    }
}
