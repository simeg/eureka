#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate clap;
extern crate dialoguer;
extern crate termcolor;

use std::io;
use std::process::Command;

use clap::ArgMatches;
use dialoguer::Select;
use termcolor::StandardStream;

use file_handler::{ConfigManagement, FileHandler, FileManagement};
use git::git::git_commit_and_push;
use printer::{Print, Printer};
use reader::{Read, Reader};
use types::CliFlag::{ClearEditor, ClearRepo, View};
use types::ConfigFile::{Editor, Repo};

pub mod file_handler;
mod git;
pub mod printer;
pub mod reader;
pub mod types;
pub mod utils;

pub struct Eureka {}

impl Eureka {
    pub fn handle_flags(&self, args: ArgMatches, fh: &FileHandler) -> Result<(), ()> {
        if args.is_present(ClearRepo.value()) {
            fh.file_rm(Repo).expect("Could not remove repo config file");
        }

        if args.is_present(ClearEditor.value()) {
            fh.file_rm(Editor)
                .expect("Could not remove editor config file");
        }

        // Exit if any "clear" flag was provided
        if args.is_present(ClearRepo.value()) || args.is_present(ClearEditor.value()) {
            return Err(());
        }

        if args.is_present(View.value()) {
            match fh.config_read(Repo) {
                Ok(repo_path) => match self.open_pager_less(repo_path) {
                    Ok(_) => {
                        return Err(());
                    }
                    Err(e) => panic!(e),
                },
                Err(e) => panic!("No path to repository found: {}", e),
            }
        }

        Ok(())
    }

    pub fn run<FH, R>(&self, fh: &FH, printer: &mut Printer<StandardStream>, reader: &mut Reader<R>)
    where
        FH: FileManagement + ConfigManagement,
        R: io::BufRead,
    {
        let repo_path = self.get_repo_path(fh, printer, reader);
        let editor_path = self.get_editor_path(fh, printer, reader);

        if !self.is_first_time_run(fh) {
            printer.print_input_header(">> Idea summary");
            let commit_msg = reader.read();
            let readme_path = format!("{}/README.md", repo_path);

            match self.open_editor(&editor_path, &readme_path) {
                Ok(_) => {
                    let _ = git_commit_and_push(&repo_path, commit_msg);
                }
                Err(e) => panic!("Could not open editor at path {}: {}", editor_path, e),
            };
        } else {
            printer.println("First time setup complete. Happy ideation!");
        }
    }

    fn get_editor_path<FH, R>(
        &self,
        fh: &FH,
        printer: &mut Printer<StandardStream>,
        reader: &mut Reader<R>,
    ) -> String
    where
        FH: FileManagement + ConfigManagement,
        R: io::BufRead,
    {
        match fh.config_read(Editor) {
            Ok(file_path) => file_path,
            Err(_) => {
                let selections = &[
                    "vim (/usr/bin/vi)",
                    "nano (/usr/bin/nano)",
                    "Other (provide path to binary)",
                ];

                printer.print_editor_selection_header();
                let index = Select::new()
                    .default(0)
                    .items(selections)
                    .interact()
                    .unwrap();

                let input_editor_path = match index {
                    0 => self.s("/usr/bin/vi"),
                    1 => self.s("/usr/bin/nano"),
                    2 => {
                        printer.print_input_header("Path to editor binary");
                        printer.flush().unwrap();
                        reader.read()
                    }
                    _ => {
                        // TODO(simeg): Do not fall back, ask user again for options
                        // TODO(simeg): How can the user even end up here?
                        printer.println("Invalid option, falling back to vim");
                        self.s("/usr/bin/vi")
                    }
                };

                if !fh.file_exists(&input_editor_path) {
                    panic!("Invalid editor path");
                }

                match fh.config_write(Editor, &input_editor_path) {
                    Ok(_) => input_editor_path,
                    Err(e) => panic!("Unable to write your editor path to disk: {}", e),
                }
            }
        }
    }

    fn get_repo_path<FH, R>(
        &self,
        fh: &FH,
        printer: &mut Printer<StandardStream>,
        reader: &mut Reader<R>,
    ) -> String
    where
        FH: FileManagement + ConfigManagement,
        R: io::BufRead,
    {
        match fh.config_read(Repo) {
            Ok(file_path) => file_path,
            Err(_) => {
                printer.print_fts_banner();
                if !fh.config_dir_exists() {
                    fh.config_dir_create()
                        .expect("Unable to create dir to store config");
                }

                // set input repo path as an empty string
                let mut input_repo_path = String::new();

                // as long as the path is empty
                while input_repo_path.is_empty() {
                    // ask for the path again...
                    printer.print_input_header("Absolute path to your idea repo");
                    printer.flush().unwrap();
                    input_repo_path = reader.read();
                }

                match fh.config_write(Repo, &input_repo_path) {
                    Ok(_) => input_repo_path,
                    Err(e) => panic!("Unable to write your repo path to disk: {}", e),
                }
            }
        }
    }

    fn is_first_time_run<FH>(&self, fh: &FH) -> bool
    where
        FH: ConfigManagement,
    {
        match fh.config_read(Repo) {
            Ok(_) => false,
            Err(_) => true,
        }
    }

    fn open_editor(&self, bin_path: &String, file_path: &String) -> io::Result<()> {
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

    fn open_pager_less(&self, repo_config_file: String) -> io::Result<()> {
        let readme_path = format!("{}/README.md", repo_config_file);
        match Command::new(self.less()).arg(&readme_path).status() {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!(
                    "Error: Could not open idea file with less at [{}]: {}",
                    readme_path, e
                );
                Err(e)
            }
        }
    }

    fn less(&self) -> String {
        utils::utils::get_if_available("less")
            .expect("Cannot locate executable - less - on your system")
    }

    fn s(&self, string: &str) -> String {
        string.to_owned()
    }
}
