extern crate termcolor;

use termcolor::WriteColor;

use std::io::{BufRead, Write};
use std::process::Command;
use std::{env, io};

use file_handler::{ConfigManagement, FileHandler, FileManagement};
use printer::{Print, Printer};
use reader::{Read, Reader};
use types::ConfigFile::Repo;
use utils::get_if_available;

pub mod file_handler;
mod git;
pub mod printer;
pub mod reader;
pub mod types;
pub mod utils;

pub struct Eureka<W, R> {
    pub fh: FileHandler,
    pub printer: Printer<W>,
    pub reader: Reader<R>,
}

impl<W, R> Eureka<W, R>
where
    W: Write + WriteColor,
    R: BufRead,
{
    pub fn run(&mut self) {
        if self.is_config_missing() {
            if self.is_first_time_run() {
                // If config dir is missing - create it
                if !self.fh.config_dir_exists() {
                    self.fh.config_dir_create().unwrap();
                }

                self.printer.print_fts_banner();
            }

            // If repo path is missing - ask for it
            if self.fh.config_read(Repo).is_err() {
                self.setup_repo_path().unwrap();
            }

            self.printer
                .print("First time setup complete. Happy ideation!");
        } else {
            self.input_idea();
        }
    }

    pub fn clear_repo(&self) {
        if self.fh.config_read(Repo).is_ok() {
            self.fh
                .file_rm(Repo)
                .expect("Could not remove repo config file");
        }
    }

    pub fn open_idea_file(&self) {
        match self.fh.config_read(Repo) {
            Ok(repo_path) => self.open_pager(repo_path).unwrap(),
            Err(e) => panic!("No path to repository found: {}", e),
        }
    }

    fn setup_repo_path(&mut self) -> io::Result<()> {
        let mut input_repo_path = String::new();

        while input_repo_path.is_empty() {
            self.printer
                .print_input_header("Absolute path to your idea repo");
            self.printer.flush().unwrap();
            input_repo_path = self.reader.read();
        }

        self.fh.config_write(Repo, input_repo_path)
    }

    fn is_first_time_run(&self) -> bool {
        self.fh.config_read(Repo).is_err()
    }

    fn is_config_missing(&self) -> bool {
        self.fh.config_read(Repo).is_err()
    }

    fn input_idea(&mut self) {
        self.printer.print_input_header(">> Idea summary");
        let idea_summary = self.reader.read();

        let repo_path = self.fh.config_read(Repo).unwrap();
        let readme_path = format!("{}/README.md", repo_path);

        match self.open_editor(&readme_path) {
            Ok(_) => git::commit_and_push(&repo_path, idea_summary).unwrap(),
            Err(e) => panic!(e),
        };
    }

    fn open_editor(&self, file_path: &str) -> io::Result<()> {
        let editor = match env::var("EDITOR") {
            Ok(e) => e,
            Err(_) => {
                get_if_available("vim").expect("Cannot locate executable - vim - on your system")
            }
        };
        match Command::new(&editor).arg(file_path).status() {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!(
                    "Error: Unable to open file [{}] with editor binary at [{}]: {}",
                    file_path, editor, e
                );
                Err(e)
            }
        }
    }

    fn open_pager(&self, repo_config_file: String) -> io::Result<()> {
        let readme_path = format!("{}/README.md", repo_config_file);
        let pager = match env::var("PAGER") {
            Ok(p) => p,
            Err(_) => {
                get_if_available("less").expect("Cannot locate executable - less - on your system")
            }
        };
        match Command::new(&pager).arg(&readme_path).status() {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!(
                    "Error: Could not open idea file with {} at [{}]: {}",
                    pager, readme_path, e
                );
                Err(e)
            }
        }
    }
}
