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
        let editor = env::var("EDITOR")
            .map(PathBuf::from)
            .or_else(|_| self.get_if_available("vi"))?;

        Command::new(&editor).arg(file_path).status().map(|_| ())
    }

    fn open_pager(&self, file_path: &str) -> io::Result<()> {
        let pager = env::var("PAGER")
            .map(PathBuf::from)
            .or_else(|_| self.get_if_available("less"))?;

        Command::new(&pager).arg(&file_path).status().map(|_| ())
    }
}

impl ProgramAccess {
    fn get_if_available(&self, program: &str) -> io::Result<PathBuf> {
        which::which(program).map_err(|err| std::io::Error::new(ErrorKind::NotFound, err))
    }
}
