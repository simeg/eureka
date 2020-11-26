use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;
use std::{env, io};

pub trait ProgramOpener {
    fn open_editor(&self, file_path: &str) -> io::Result<()>;
    fn open_pager(&self, file_path: &str) -> io::Result<()>;
}

pub struct ProgramAccess;

impl Default for ProgramAccess {
    fn default() -> Self {
        Self {}
    }
}

impl ProgramOpener for ProgramAccess {
    fn open_editor(&self, file_path: &str) -> io::Result<()> {
        self.open_with_fallback(&file_path, "EDITOR", "vi")
    }

    fn open_pager(&self, file_path: &str) -> io::Result<()> {
        self.open_with_fallback(&file_path, "PAGER", "less")
    }
}

impl ProgramAccess {
    fn open_with_fallback(&self, file_path: &str, env_var: &str, fallback: &str) -> io::Result<()> {
        let program = env::var(env_var)
            .map(PathBuf::from)
            .or_else(|_| self.get_if_available(fallback))?;

        Command::new(&program).arg(&file_path).status().map(|_| ())
    }

    fn get_if_available(&self, program: &str) -> io::Result<PathBuf> {
        which::which(program).map_err(|err| std::io::Error::new(ErrorKind::NotFound, err))
    }
}
