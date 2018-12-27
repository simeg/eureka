#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
extern crate clap;
extern crate dialoguer;
extern crate termcolor;

use std::io;
use std::process::Command;

use clap::{App, Arg, ArgMatches};
use dialoguer::Select;
use termcolor::{ColorChoice, StandardStream};

use file_handler::{ConfigManagement, FileHandler, FileManagement};
use git::git::git_commit_and_push;
use printer::{Print, Printer};
use reader::{Read, Reader};
use types::CliFlag::{ClearEditor, ClearRepo, ShortView, View};
use types::ConfigType::{Editor, Repo};
use utils::utils::exit_w_code;

mod file_handler;
mod git;
mod printer;
mod reader;
mod types;
mod utils;

fn main() {
    let cli_flags: ArgMatches = App::new("eureka")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Input and store your ideas without leaving the terminal")
        .arg(
            Arg::with_name(ClearRepo.value())
                .long(ClearRepo.value())
                .help("Clear the stored path to your idea repo"),
        )
        .arg(
            Arg::with_name(ClearEditor.value())
                .long(ClearEditor.value())
                .help("Clear the stored path to your idea editor"),
        )
        .arg(
            Arg::with_name(View.value())
                .long(View.value())
                .short(ShortView.value())
                .help("View your ideas using less"),
        )
        .get_matches();

    let stdio = io::stdin();
    let input = stdio.lock();
    let output = StandardStream::stdout(ColorChoice::AlwaysAnsi);

    let mut p = Printer { writer: output };
    let mut r = Reader { reader: input };
    let fh = FileHandler {};

    if handle_flags(cli_flags, &fh).is_none() {
        exit_w_code(0);
    }

    run(&fh, &mut p, &mut r);
}

fn handle_flags(args: ArgMatches, fh: &FileHandler) -> Option<()> {
    if args.is_present(ClearRepo.value()) {
        fh.file_rm(Repo).expect("Could not remove repo config file");
    }

    if args.is_present(ClearEditor.value()) {
        fh.file_rm(Editor)
            .expect("Could not remove editor config file");
    }

    // Exit if any "clear" flag was provided
    if args.is_present(ClearRepo.value()) || args.is_present(ClearEditor.value()) {
        return None;
    }

    if args.is_present(View.value()) {
        match fh.config_read(Repo) {
            Ok(repo_path) => match open_pager_less(repo_path) {
                Ok(_) => {
                    return None;
                }
                Err(e) => panic!(e),
            },
            Err(e) => panic!("No path to repository found: {}", e),
        }
    }

    Some(())
}

fn run<FH, R>(fh: &FH, printer: &mut Printer<StandardStream>, reader: &mut Reader<R>)
where
    FH: FileManagement + ConfigManagement,
    R: io::BufRead,
{
    let repo_path = get_repo_path(fh, printer, reader);
    let editor_path = get_editor_path(fh, printer, reader);

    if !is_first_time_run(fh) {
        printer.print_input_header(">> Idea summary");
        let commit_msg = reader.read();
        let readme_path = format!("{}/README.md", repo_path);

        match open_editor(&editor_path, &readme_path) {
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

            let input_path: String = match index {
                0 => s("/usr/bin/vi"),
                1 => s("/usr/bin/nano"),
                2 => {
                    printer.print_input_header("Path to editor binary");
                    printer.flush().unwrap();
                    reader.read()
                }
                _ => {
                    // TODO(simeg): Do not fall back, ask user again for options
                    // TODO(simeg): How can the user even end up here?
                    p.println("Invalid option, falling back to vim");
                    s("/usr/bin/vi")
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

            printer.print_input_header("Absolute path to your idea repo");
            printer.flush().unwrap();
            // TODO(simeg): Re-try if path is empty, or simply just don't accept empty input
            let input_repo_path = reader.read();

            match fh.config_write(Repo, &input_repo_path) {
                Ok(_) => input_repo_path,
                Err(e) => panic!("Unable to write your repo path to disk: {}", e),
            }
        }
    }
}

fn is_first_time_run<FH>(fh: &FH) -> bool
where
    FH: ConfigManagement,
{
    match fh.config_read(Repo) {
        Ok(_) => false,
        Err(_) => true,
    }
}

fn open_editor(bin_path: &String, file_path: &String) -> io::Result<()> {
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

fn open_pager_less(repo_config_file: String) -> io::Result<()> {
    let readme_path = format!("{}/README.md", repo_config_file);
    match Command::new(less()).arg(&readme_path).status() {
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

fn less() -> String {
    if utils::utils::is_available("less") {
        String::from("less")
    } else {
        panic!("Cannot locate executable - less - on your system")
    }
}

fn s(string: &str) -> String {
    string.to_owned()
}

#[cfg(test)]
mod tests {
    use std::io::Error;

    use reader::Reader;
    use types::ConfigType;

    use super::*;

    struct MockFileHandler;

    impl FileManagement for MockFileHandler {
        fn create_dir(&self, path: &str) -> Result<(), Error> {
            unimplemented!()
        }

        fn file_exists(&self, path: &str) -> bool {
            unimplemented!()
        }

        fn file_rm(&self, file: ConfigType) -> Result<(), Error> {
            unimplemented!()
        }
    }

    impl ConfigManagement for MockFileHandler {
        fn config_dir_create(&self) -> Result<String, Error> {
            unimplemented!()
        }

        fn config_dir_exists(&self) -> bool {
            unimplemented!()
        }

        fn config_read(&self, file: ConfigType) -> Result<String, Error> {
            unimplemented!()
        }

        fn config_write(&self, file: ConfigType, value: &String) -> Result<(), Error> {
            unimplemented!()
        }
    }

    #[test]
    fn tests_work() {
        let fh = MockFileHandler {};
        //        let mut output = Vec::new();
        let mut output = StandardStream::stdout(ColorChoice::AlwaysAnsi);
        let input = b"I'm George";
        let mut reader = Reader { reader: &input[..] };
        let mut printer = Printer { writer: output };
        run(&fh, &mut printer, &mut reader);

        assert_eq!(true, true);
    }
}
