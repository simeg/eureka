pub mod file_handler {
    extern crate serde;
    extern crate serde_json as json;

    use std::error::Error;
    use std::fs;
    use std::io;
    use std::io::ErrorKind;
    use std::io::{Read, Write};
    use std::path::Path;

    pub fn path_exists(path: &str) -> bool {
        fs::metadata(path).is_ok()
    }

    pub fn read_from_config(path: String) -> io::Result<String> {
        let config_path = format!(
            "{location}/{path}",
            location = config_location(),
            path = path
        );
        let mut file = fs::File::open(&config_path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect(&format!("Unable to read file at: {}", config_path));
        if contents.ends_with("\n") {
            contents.pop().expect("File is empty");
        }
        Ok(contents)
    }

    pub fn write_to_config_json<T: ::serde::Serialize>(
        key: &str,
        data: T,
    ) -> Result<T, (json::Error)> {
        let location = config_location();
        let path = format!("{}/{}", location, key);
        match ::fs::File::create(path) {
            Ok(mut file) => match ::json::to_string::<T>(&data) {
                Ok(str_data) => {
                    let _ = file.write(&str_data.replace("\"", "").into_bytes());
                    Ok(data)
                }
                Err(e) => Err(e),
            },
            Err(_) => {
                // TODO: Overwrite existing value, use additional param to decide it
                // File for [key] already exist, doing nothing
                Ok(data)
            }
        }
    }

    pub fn write_to_config(file_name: String, value: String) -> io::Result<()> {
        let path_to_write_to = format!(
            "{location}/{file_name}",
            location = config_location(),
            file_name = file_name
        );

        let path = Path::new(&path_to_write_to);

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

    pub fn rm_config_file(file_name: String) -> io::Result<()> {
        let config_path = format!(
            "{location}/{file}",
            location = config_location(),
            file = file_name
        );
        rm_file(config_path)?;
        Ok(())
    }

    fn rm_file(path: String) -> io::Result<()> {
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
