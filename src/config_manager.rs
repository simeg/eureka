use crate::dirs::home_dir;

use std::env::var;
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;
use std::{fs, io};

use serde::{Deserialize, Serialize};

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize, Default)]
struct Config {
    repo: PathBuf,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ConfigType {
    Repo,
}

pub trait ConfigManagement {
    fn config_dir_create(&self) -> io::Result<()>;
    fn config_dir_exists(&self) -> bool;
    fn config_read(&self, config_type: ConfigType) -> io::Result<String>;
    fn config_write(&self, config_type: ConfigType, value: String) -> io::Result<()>;
    fn config_rm(&self) -> io::Result<()>;
}

#[derive(Default)]
pub struct ConfigManager;

impl ConfigManagement for ConfigManager {
    fn config_dir_create(&self) -> io::Result<()> {
        self.config_dir_path().and_then(fs::create_dir_all)
    }

    fn config_dir_exists(&self) -> bool {
        self.config_dir_path().and_then(fs::metadata).is_ok()
    }

    fn config_read(&self, config_type: ConfigType) -> io::Result<String> {
        let config = self.config()?;
        let config_value = match config_type {
            ConfigType::Repo => config.repo.display().to_string(),
        };
        Ok(config_value)
    }

    fn config_write(&self, config_type: ConfigType, value: String) -> io::Result<()> {
        let config_path = self.config_path()?;

        // Create file if it doesn't exist, otherwise get it
        let mut file = fs::File::create(config_path)?;

        let mut config = self.config()?;
        match config_type {
            ConfigType::Repo => config.repo = PathBuf::from(value),
        }

        let json = serde_json::to_string(&config)?;

        file.write_all(json.as_bytes())
    }

    fn config_rm(&self) -> io::Result<()> {
        let config_path = self.config_path()?;
        // Make sure file exists
        fs::metadata(&config_path)?;
        fs::remove_file(&config_path)
    }
}

impl ConfigManager {
    fn config_path(&self) -> io::Result<PathBuf> {
        Ok(self.config_dir_path()?.join(CONFIG_FILE_NAME))
    }

    fn config_dir_path(&self) -> io::Result<PathBuf> {
        self.resolve_xdg_config_home()
            .or_else(|| Some(home_dir().unwrap().join(".config").join("eureka")))
            .ok_or_else(|| {
                io::Error::new(
                    ErrorKind::NotFound,
                    "Could not resolve your $HOME directory",
                )
            })
    }

    fn config(&self) -> io::Result<Config> {
        let config_file = self.config_path()?;
        // Make sure file exists
        fs::metadata(&config_file)?;

        let mut file = fs::File::open(&config_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        if contents.is_empty() {
            return Ok(Config::default());
        }

        Ok(serde_json::from_str(&contents)?)
    }

    fn resolve_xdg_config_home(&self) -> Option<PathBuf> {
        match var("XDG_CONFIG_HOME") {
            Ok(path) => Some(PathBuf::from(path).join("eureka")),
            Err(_) => None,
        }
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use crate::config_manager::{ConfigManagement, ConfigManager, ConfigType};
    use std::io::{Read, Write};
    use std::path::{Path, PathBuf};
    use std::{env, fs, io, path};
    use tempfile::TempDir;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_config_manager__config_dir_path() -> TestResult {
        let cm = ConfigManager::default();
        let (_config_dir, tmp_dir) = set_config_dir()?;

        // XDG_CONFIG_HOME is set in Github Actions so let's unset it
        env::remove_var("XDG_CONFIG_HOME");
        assert!(env::var("XDG_CONFIG_HOME").is_err());

        let actual = cm.config_dir_path()?;
        let expected = tmp_dir.path().join(".config").join("eureka");

        env::remove_var("HOME");

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_config_manager__config_dir_path__when__xdg_config_home_env_var_set() -> TestResult {
        use std::path::Path;

        let cm = ConfigManager::default();
        env::set_var("XDG_CONFIG_HOME", "/specific-path/.config");
        assert_eq!(
            env::var("XDG_CONFIG_HOME"),
            Ok(String::from("/specific-path/.config"))
        );

        let actual = cm.config_dir_path()?;
        let expected = Path::new("/specific-path").join(".config").join("eureka");

        env::remove_var("XDG_CONFIG_HOME");
        assert!(env::var("XDG_CONFIG_HOME").is_err());

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_config_manager__config_dir_create() -> TestResult {
        let cm = ConfigManager::default();
        let (_config_dir, _tmp_dir) = set_config_dir()?;

        let actual = cm.config_dir_create();

        env::remove_var("HOME");

        assert!(actual.is_ok());
        Ok(())
    }

    #[test]
    fn test_config_manager__config_dir_exists__success() -> TestResult {
        let cm = ConfigManager::default();
        let (_config_dir, _tmp_dir) = set_and_create_config_dir()?;

        let config_dir_exists = cm.config_dir_exists();

        env::remove_var("HOME");

        assert!(config_dir_exists);
        Ok(())
    }

    #[test]
    fn test_config_manager__config_dir_exists__failure() -> TestResult {
        let cm = ConfigManager::default();
        let (_config_dir, _tmp_dir) = set_config_dir()?;

        // XDG_CONFIG_HOME is set in Github Actions so let's unset it
        env::remove_var("XDG_CONFIG_HOME");
        assert!(env::var("XDG_CONFIG_HOME").is_err());

        let config_dir_exists = cm.config_dir_exists();

        env::remove_var("HOME");

        assert!(!config_dir_exists);
        Ok(())
    }

    #[test]
    fn test_config_manager__config_read__success() -> TestResult {
        let cm = ConfigManager::default();
        let (config_dir, _tmp_dir) = set_and_create_config_dir()?;
        let mut file =
            fs::File::create(path::Path::new(&config_dir.join("config.json").as_os_str()))?;
        file.write_all("{\"repo\": \"this-repo-path-value\"}".as_bytes())?;

        let actual = cm.config_read(ConfigType::Repo)?;
        let expected = "this-repo-path-value";

        env::remove_var("HOME");

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_config_manager__config_read__file_is_empty__default_config() -> TestResult {
        let cm = ConfigManager::default();
        let (config_dir, _tmp_dir) = set_and_create_config_dir()?;
        // Create file but leave it empty
        let _file = fs::File::create(path::Path::new(&config_dir.join("config.json").as_os_str()))?;

        let actual = cm.config_read(ConfigType::Repo)?;
        let expected = String::from("");

        env::remove_var("HOME");

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_config_manager__config_read__when__file_does_not_exist__failure() -> TestResult {
        let cm = ConfigManager::default();
        let (_config_dir, _tmp_dir) = set_and_create_config_dir()?;

        let actual = cm.config_read(ConfigType::Repo).map_err(|e| e.kind());
        let expected = Err(io::ErrorKind::NotFound);

        env::remove_var("HOME");

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_config_manager__config_write__config_file_does_not_already_exist__success() -> TestResult
    {
        let cm = ConfigManager::default();
        let (config_dir, _tmp_dir) = set_and_create_config_dir()?;

        let write_result = cm.config_write(ConfigType::Repo, String::from("this-specific-value"));

        env::remove_var("HOME");

        assert!(write_result.is_ok());

        // Assert file contents
        let contents = get_file_contents(&config_dir)?;
        let expected = "{\"repo\":\"this-specific-value\"}";

        assert_eq!(contents, expected);
        Ok(())
    }

    #[test]
    fn test_config_manager__config_write__config_file_already_exists__success() -> TestResult {
        let cm = ConfigManager::default();
        let (config_dir, _tmp_dir) = set_and_create_config_dir()?;
        // Create file but leave it empty
        let _file = fs::File::create(path::Path::new(&config_dir.join("config.json").as_os_str()))?;

        let write_result = cm.config_write(ConfigType::Repo, String::from("this-specific-value"));

        env::remove_var("HOME");

        assert!(write_result.is_ok());

        // Assert file contents
        let mut file = fs::File::open(config_dir.join("config.json"))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let expected = "{\"repo\":\"this-specific-value\"}";

        assert_eq!(contents, expected);
        Ok(())
    }

    #[test]
    fn test_config_manager__config_rm__success() -> TestResult {
        let cm = ConfigManager::default();
        let (config_dir, _tmp_dir) = set_and_create_config_dir()?;
        // Create file but leave it empty
        let _file = fs::File::create(path::Path::new(&config_dir.join("config.json").as_os_str()))?;

        let actual = cm.config_rm();

        env::remove_var("HOME");

        assert!(actual.is_ok());
        Ok(())
    }

    #[test]
    fn test_config_manager__config_rm__file_does_not_exist__failure() -> TestResult {
        let cm = ConfigManager::default();
        let (_config_dir, _tmp_dir) = set_and_create_config_dir()?;

        let actual = cm.config_rm().map_err(|e| e.kind());
        let expected = Err(io::ErrorKind::NotFound);

        env::remove_var("HOME");

        assert_eq!(actual, expected);
        Ok(())
    }

    fn set_config_dir() -> io::Result<(PathBuf, TempDir)> {
        let tmp_dir = TempDir::new()?;
        // Create the config dir. When tmp_dir is destroyed it will be deleted
        let config_dir = tmp_dir.path().join(".config").join("eureka");

        env::set_var("HOME", tmp_dir.path());
        assert_eq!(
            env::var("HOME"),
            Ok(tmp_dir.path().to_str().unwrap().to_string())
        );

        // tmp_dir cannot be destroyed yet, so return it
        Ok((config_dir, tmp_dir))
    }

    fn set_and_create_config_dir() -> io::Result<(PathBuf, TempDir)> {
        let (config_dir, tmp_dir) = set_config_dir()?;

        fs::create_dir_all(&config_dir)?;

        // tmp_dir cannot be destroyed yet, so return it
        Ok((config_dir, tmp_dir))
    }

    fn get_file_contents(config_dir: &Path) -> io::Result<String> {
        let mut file = fs::File::open(config_dir.join("config.json"))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
