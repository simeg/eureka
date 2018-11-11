#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
extern crate clap;
extern crate dialoguer;
#[macro_use]
extern crate text_io;

use clap::ArgMatches;
use clap::{App, Arg};
use dialoguer::Select;
use file_handler::Config::*;
use file_handler::ConfigManagement;
use file_handler::FileHandler;
use file_handler::FileManagement;
use git::git::git_commit_and_push;
use std::io;
use std::io::{Error, Write};
use std::process::Command;

mod file_handler;
mod git;
mod utils;

fn main() {
    let cli_flags: ArgMatches = App::new("eureka")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Input and store your ideas without leaving the terminal")
        // TODO(simeg): Keep argument strings DRY
        .arg(
            Arg::with_name("clear-repo")
                .long("clear-repo")
                .help("Clear the stored path to your idea repo"),
        )
        .arg(
            Arg::with_name("clear-editor")
                .long("clear-editor")
                .help("Clear the stored path to your idea editor"),
        )
        .arg(
            Arg::with_name("view")
                .long("view")
                .help("View your ideas using less"),
        )
        .get_matches();

    let fh = FileHandler {};

    if cli_flags.is_present("clear-repo") {
        match fh.file_rm(Repo) {
            Ok(_) => {}
            Err(e) => panic!(e),
        }
    }

    if cli_flags.is_present("clear-editor") {
        match fh.file_rm(Editor) {
            Ok(_) => {}
            Err(e) => panic!(e),
        }
    }

    if cli_flags.is_present("view") {
        match fh.config_read(Repo) {
            Ok(repo_path) => {
                if repo_path.is_empty() {
                    panic!("No path to repository found");
                } else {
                    match open_pager_less(repo_path) {
                        Ok(_) => {
                            ::std::process::exit(0);
                        }
                        Err(e) => panic!(e),
                    }
                }
            }
            Err(_) => panic!("No path to repository found"),
        }
    }

    // If first time setup is carried out do not
    // require an idea to be written down.
    let mut first_time: bool = false;

    let repo_path: String = match fh.config_read(Repo) {
        Ok(file_path) => file_path,
        Err(_) => {
            first_time = true;
            display_first_time_setup_banner();
            if !fh.config_dir_exists() {
                fh.config_dir_create()
                    .expect("Unable to create dir to store config");
            }

            print!("Absolute path to your idea repo: ");
            io::stdout().flush().unwrap();
            let input_path: String = read!();
            let copy_input_path: String = input_path.clone();

            match fh.config_write(Repo, input_path) {
                Ok(_) => copy_input_path,
                Err(e) => panic!("Unable to write your repo path to disk: {}", e),
            }
        }
    };

    let editor_path: String = match fh.config_read(Editor) {
        Ok(file_path) => file_path,
        Err(_) => {
            let selections = &[
                "vim (/usr/bin/vim)",
                "nano (/usr/bin/nano)",
                "Other (provide path to binary)",
            ];

            println!("What editor do you want to use for writing down your ideas?");
            let index = Select::new()
                .default(0)
                .items(selections)
                .interact()
                .unwrap();

            let input_path: String = match index {
                0 => s("/usr/bin/vim"),
                1 => s("/usr/bin/nano"),
                2 => {
                    print!("Path to editor binary: ");
                    io::stdout().flush().unwrap();
                    let editor_bin_path: String = read!();
                    editor_bin_path
                }
                _ => {
                    // TODO: Do not fall back, ask user again for options
                    println!("Invalid option, falling back to vim");
                    s("/usr/bin/vim")
                }
            };

            if !fh.file_exists(&input_path) {
                panic!("Invalid editor path");
            }

            let copy_input_path: String = input_path.clone();
            match fh.config_write(Editor, input_path) {
                Ok(_) => copy_input_path,
                Err(e) => panic!("Unable to write your editor path to disk: {}", e),
            }
        }
    };

    if !first_time {
        let commit_msg: String = get_commit_msg();
        let readme_path: String = format!("{}/README.md", repo_path);

        match open_editor(&editor_path, &readme_path) {
            Ok(_) => {
                let _ = git_commit_and_push(&repo_path, commit_msg);
            }
            Err(e) => panic!("Could not open editor at path {}: {}", editor_path, e),
        };
    } else {
        println!("First time setup complete. Happy ideation!")
    }
}

fn display_first_time_setup_banner() {
    println!();
    println!("##########################################################");
    println!("####                 First Time Setup                 ####");
    println!("##########################################################");
    println!();
    println!("This tool requires you to have a repository with a README.md");
    println!("in the root folder. The markdown file is where your ideas will");
    println!("be stored. ");
    println!();
    println!("Once first time setup has completed, simply run Eureka again");
    println!("to begin writing down ideas.");
    println!();
}

fn get_commit_msg() -> String {
    println!("Idea commit subject: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn open_editor(bin_path: &String, file_path: &String) -> Result<(), Error> {
    match Command::new(bin_path).arg(file_path).status() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!(
                "Unable to open file [{}] with editor binary at [{}]: {}",
                file_path, bin_path, e
            );
            Err(e)
        }
    }
}

fn open_pager_less(repo_config_file: String) -> Result<(), Error> {
    let readme_path = &(repo_config_file + "/README.md");
    match Command::new(less()).arg(readme_path).status() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!(
                "Could not open idea file with less at [{}]: {}",
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

/*
 * Helpers
*/

fn s(string: &str) -> String {
    string.to_owned()
}
