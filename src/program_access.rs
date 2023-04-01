use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs, io};

pub trait ProgramOpener {
    fn open_editor(&self, file_path: &str) -> io::Result<()>;
    fn open_pager(&self, file_path: &str) -> io::Result<()>;
}

#[derive(Default)]
pub struct ProgramAccess;

impl ProgramOpener for ProgramAccess {
    fn open_editor(&self, file_path: &str) -> io::Result<()> {
        self.open_with_fallback(file_path, "EDITOR", "vi")
    }

    fn open_pager(&self, file_path: &str) -> io::Result<()> {
        self.open_with_fallback(file_path, "PAGER", "less")
    }
}

impl ProgramAccess {
    fn open_with_fallback(&self, file_path: &str, env_var: &str, fallback: &str) -> io::Result<()> {
        let program = env::var(env_var)
            .map(PathBuf::from)
            .or_else(|_| self.get_if_available(fallback))?;

        // Make sure file exists
        fs::metadata(file_path)?;
        Command::new(program).arg(file_path).status().map(|_| ())
    }

    fn get_if_available(&self, program: &str) -> io::Result<PathBuf> {
        which::which(program).map_err(|err| std::io::Error::new(ErrorKind::NotFound, err))
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use crate::program_access::{ProgramAccess, ProgramOpener};
    use std::env;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_program_access__get_if_available__success() {
        let program_access = ProgramAccess::default();

        let actual = program_access.get_if_available("echo");

        assert!(actual.is_ok())
    }

    #[test]
    fn test_program_access__get_if_available__failure() {
        let program_access = ProgramAccess::default();

        let actual = program_access.get_if_available("some-non-existing-program");

        assert!(actual.is_err())
    }

    #[test]
    fn test_program_access__open_with_fallback__success() -> TestResult {
        let program_access = ProgramAccess::default();
        let tmp_file = tempfile::NamedTempFile::new()?;
        let file_path = tmp_file.path().to_str().unwrap();
        env::set_var("READER_ENV_VAR", "echo");

        program_access.open_with_fallback(
            file_path,
            "READER_ENV_VAR",
            "some-non-existing-program",
        )?;

        env::remove_var("READER_ENV_VARIABLE");
        Ok(())
    }

    #[test]
    fn test_program_access__open_with_fallback__uses_fallback() -> TestResult {
        let program_access = ProgramAccess::default();
        let tmp_file = tempfile::NamedTempFile::new()?;
        let file_path = tmp_file.path().to_str().unwrap();
        env::remove_var("THIS_ENV_VAR");

        program_access.open_with_fallback(file_path, "THIS_ENV_VAR", "echo")?;

        Ok(())
    }

    #[test]
    fn test_program_access__open_editor__success() -> TestResult {
        let program_access = ProgramAccess::default();
        let tmp_file = tempfile::NamedTempFile::new()?;
        let file_path = tmp_file.path().to_str().unwrap();
        let editor_value = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
        env::set_var("EDITOR", "echo");

        program_access.open_editor(file_path)?;

        env::set_var("EDITOR", editor_value);
        Ok(())
    }

    #[test]
    fn test_program_access__open_pager__success() -> TestResult {
        let program_access = ProgramAccess::default();
        let tmp_file = tempfile::NamedTempFile::new()?;
        let file_path = tmp_file.path().to_str().unwrap();
        let pager_value = env::var("PAGER").unwrap_or_else(|_| "less".to_string());
        env::set_var("PAGER", "echo");

        program_access.open_pager(file_path)?;

        env::set_var("PAGER", pager_value);
        Ok(())
    }
}
