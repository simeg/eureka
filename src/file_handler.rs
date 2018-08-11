pub mod file_handler {
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

    impl ConfigFile {
        pub fn name(&self) -> String {
            match *self {
                ConfigFile::Repo => config_path(CONFIG_REPO.to_string()),
                ConfigFile::Editor => config_path(CONFIG_EDITOR.to_string()),
            }
        }
    }

    fn config_path(file_name: String) -> String {
        match ::env::home_dir() {
            Some(location) => format!(
                "{home}/{eureka}/{file_name}",
                home = location.display(),
                eureka = ".eureka",
                file_name = file_name
            ),
            None => panic!("Could not resolve your $HOME directory"),
        }
    }

    pub fn path_exists(path: &str) -> bool {
        fs::metadata(path).is_ok()
    }

    pub fn read_from_config(file_name: String) -> io::Result<String> {
        let mut file = fs::File::open(&file_name)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect(&format!("Unable to read file at: {}", file_name));
        if contents.ends_with("\n") {
            contents.pop().expect("File is empty");
        }

        Ok(contents)
    }

    pub fn write_to_config(file_name: String, value: String) -> io::Result<()> {
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

    pub fn config_location() -> String {
        match ::env::home_dir() {
            Some(location) => format!("{}/{}", location.display(), ".eureka"),
            None => panic!("Could not resolve your $HOME directory"),
        }
    }

    pub fn rm_file(path: String) -> io::Result<()> {
        if path_exists(&path) {
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
