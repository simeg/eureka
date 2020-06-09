extern crate dialoguer;
extern crate termcolor;

use dialoguer::Select;
use termcolor::WriteColor;

use std::io::{BufRead, Write};
use std::process::Command;
use std::{env, io};

use file_handler::{ConfigManagement, FileHandler, FileManagement};
use printer::{Print, Printer};
use reader::{Read, Reader};
use types::ConfigFile::{Editor, Repo};
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

            // If editor path is missing - ask for it
            if self.fh.config_read(Editor).is_err() {
                self.setup_editor_path().unwrap();
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

    pub fn clear_editor(&self) {
        if self.fh.config_read(Editor).is_ok() {
            self.fh
                .file_rm(Editor)
                .expect("Could not remove editor config file");
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

    fn setup_editor_path(&mut self) -> io::Result<()> {
        self.printer.print_editor_selection_header();

        let select_index = Select::new()
            .default(0)
            .items(&["vim", "nano", "Other (provide name, e.g. 'emacs')"])
            .interact()
            .unwrap();

        let chosen_editor = match select_index {
            0 => "vim".to_string(),
            1 => "nano".to_string(),
            2 => {
                self.printer.print_input_header("");
                self.printer.flush().unwrap();
                self.reader.read()
            }
            _ => panic!("You should not be able to get here"),
        };

        let editor_path = get_if_available(chosen_editor.as_str()).unwrap_or_else(|| {
            panic!("Could not find executable for {} - aborting", chosen_editor)
        });

        self.fh.config_write(Editor, editor_path)
    }

    fn is_first_time_run(&self) -> bool {
        self.fh.config_read(Repo).is_err() && self.fh.config_read(Editor).is_err()
    }

    fn is_config_missing(&self) -> bool {
        self.fh.config_read(Repo).is_err() || self.fh.config_read(Editor).is_err()
    }

    fn input_idea(&mut self) {
        self.printer.print_input_header(">> Idea summary");
        let idea_summary = self.reader.read();

        let editor_path = self.fh.config_read(Editor).unwrap();
        let repo_path = self.fh.config_read(Repo).unwrap();
        let readme_path = format!("{}/README.md", repo_path);

        match self.open_editor(&editor_path, &readme_path) {
            Ok(_) => git::commit_and_push(&repo_path, idea_summary).unwrap(),
            Err(e) => panic!("Could not open editor at path {}: {}", editor_path, e),
        };
    }

    fn open_editor(&self, bin_path: &str, file_path: &str) -> io::Result<()> {
        match Command::new(bin_path).arg(file_path).status() {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!(
                    "Error: Unable to open file [{}] with editor binary at [{}]: {}",
                    file_path, bin_path, e
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
