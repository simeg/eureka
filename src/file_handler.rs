extern crate dirs;

use self::dirs::home_dir;
use std::error::Error;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::io::{Read, Write};
use std::path;
use utils::utils;
use git::git;

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

pub trait IdeaManagement {
    fn get_idea_path(&self, repo_path: &String, path: &String) -> io::Result<(String, bool)>;
    fn add_idea_to_readme(&self, readme_path: &String, commit_msg: &String, repo_path: &String) -> io::Result<()>;
}

impl IdeaManagement for FileHandler {
    fn get_idea_path(&self, repo_path: &String, path: &String) -> io::Result<(String, bool)> {
        let folder_path: String = format!("{}/ideas", repo_path);
        let path: String = format!("{}/{}.md", folder_path, utils::format_idea_filename(path));

        if !self.file_exists(&folder_path.as_str()) {
            let result = fs::create_dir(&folder_path);
            if result.is_err() {
                let error = result.unwrap_err();
                return Err(io::Error::new(error.kind(), format!("Could not create directory {} : {}",
                                                  folder_path,
                                                  error)))
            }
        }

        if !self.file_exists(&path.as_str()) {
            match fs::File::create(&path) {
                Ok(_) => Ok((path, true)),
                // Re-constructing the error like this allows us to add our own text to the message.
                Err(e) => Err(io::Error::new(e.kind(), format!("Could not create file {}/{}.md : {}",
                                                  repo_path,
                                                  utils::format_idea_filename(&path),
                                                  e)))
            }
        } else {
            Ok((path, false))
        }
    }

    fn add_idea_to_readme(&self, readme_path: &String, commit_msg: &String, repo_path: &String) -> io::Result<()> {
        match fs::OpenOptions::new()
                              .write(true)
                              .append(true)
                              .open(readme_path)
        {
            Ok(mut file) => {
                let git_url = String::from_utf8(git::get_repo_url(repo_path)
                                                    .expect("Failed to get remote url"))
                                     .expect("Failed to convert url string").replace("\n", "");

                // TODO: Branches?
                match file.write(format!("## [{}]({}/blob/master/ideas/{}.md)\n",
                                       commit_msg,
                                       git_url,
                                       utils::format_idea_filename(commit_msg))
                                  .as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e)
                }
            },
            Err(e) => Err(e),
        }
    }
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
    use file_handler::FileSystem;
    use std::io;

    struct MockFileSystem;
    struct MockFileHandler;

    impl FileSystem for MockFileSystem {
        fn create_dir(&self, _path: &str) -> io::Result<()> {
            Ok(())
        }
    }

    impl ConfigManagement for MockFileHandler {
        fn config_dir_create(&self) -> io::Result<String> {
            Ok(String::from("irrelevant"))
        }

        fn config_dir_exists(&self) -> bool {
            self.file_exists(&String::from("irrelevant"))
        }

        fn config_read(&self, _file: ConfigFile) -> io::Result<String> {
            Ok(String::from("irrelevant"))
        }

        fn config_write(&self, _file: ConfigFile, _value: String) -> io::Result<()> {
            Ok(())
        }
    }

    impl FileManagement for MockFileHandler {
        fn file_exists(&self, _path: &str) -> bool {
            true
        }

        fn file_rm(&self, _file: ConfigFile) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn file_system_create_dir_is_ok() {
        let fs = MockFileSystem {};
        let result = fs.create_dir("irrelevant");
        assert!(result.is_ok());
    }

    #[test]
    fn file_handler_create_config_dir_is_ok() {
        let fh = MockFileHandler {};
        let result = fh.config_dir_create();
        assert!(result.is_ok());
    }

    #[test]
    fn file_handler_write_config_file_is_ok() {
        let fh = MockFileHandler {};
        let result = fh.config_write(ConfigFile::Repo, String::from("irrelevant"));
        assert!(result.is_ok());
    }

    #[test]
    fn file_handler_read_config_file_is_ok() {
        let fh = MockFileHandler {};
        let result = fh.config_read(ConfigFile::Repo);
        assert!(result.is_ok());
    }

    #[test]
    fn file_handler_delete_config_file_is_ok() {
        let fh = MockFileHandler {};
        let result = fh.file_rm(ConfigFile::Repo);
        assert!(result.is_ok());
    }
}
