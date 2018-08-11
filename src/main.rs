#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use std::env;
use std::fs;
use std::io;
use std::io::{Error, Write};
use std::process::Command;

extern crate serde;
extern crate serde_json as json;

#[macro_use]
extern crate text_io;

#[macro_use]
extern crate clap;

use clap::ArgMatches;
use clap::{App, Arg};
use file_handler::file_handler as fh;
use git::git::git_commit_and_push;

mod file_handler;
mod git;

fn main() {
    let cli_flags: ArgMatches = App::new("eureka")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Input and store your ideas without leaving the terminal")
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
        .get_matches();

    if cli_flags.is_present("clear-repo") {
        match fh::rm_config_file(s("repo_path")) {
            Ok(_) => {}
            Err(e) => panic!(e),
        }
    }

    if cli_flags.is_present("clear-editor") {
        match fh::rm_config_file(s("editor_path")) {
            Ok(_) => {}
            Err(e) => panic!(e),
        }
    }

    let repo_path: String = match fh::read_from_config(s("repo_path")) {
        Ok(file_path) => file_path,
        Err(_) => {
            display_first_time_setup_banner();
            if !fh::path_exists(&fh::config_location()) {
                match fs::create_dir_all(&fh::config_location()) {
                    Ok(_) => {}
                    Err(_) => panic!(
                        "Could not create dir at {} to store necessary config",
                        fh::config_location()
                    ),
                }
            }

            print!("Absolute path to your idea repo: ");
            io::stdout().flush().unwrap();
            let input_path: String = read!();
            let copy_input_path: String = input_path.clone();

            match fh::write_to_config("repo_path", input_path) {
                Ok(_) => copy_input_path,
                Err(e) => panic!("Unable to write your repo path to disk: {}", e),
            }
        }
    };

    let editor_path: String = match fh::read_from_config(s("editor_path")) {
        Ok(file_path) => file_path,
        Err(_) => {
            println!("What editor do you want to use for writing down your ideas?");
            println!("1) vim (/usr/bin/vim)");
            println!("2) nano (/usr/bin/nano)");
            println!("3) Other (provide path to binary)");
            println!();
            print!("Alternative: ");
            io::stdout().flush().unwrap();

            let input_choice: String = read!();
            // Cast to int to be able to match
            let editor_choice: u32 = input_choice.parse::<u32>().unwrap();
            let input_path: String = match editor_choice {
                1 => s("/usr/bin/vim"),
                2 => s("/usr/bin/nano"),
                3 => {
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

            if !fh::path_exists(&input_path) {
                panic!("Invalid editor path");
            }

            let copy_input_path: String = input_path.clone();
            match fh::write_to_config("editor_path", input_path) {
                Ok(_) => copy_input_path,
                Err(e) => panic!("Unable to write your editor path to disk: {}", e),
            }
        }
    };

    let commit_msg: String = get_commit_msg();
    let readme_path: String = format!("{}/README.md", repo_path);

    match open_editor(&editor_path, &readme_path) {
        Ok(_) => {
            let _ = git_commit_and_push(&repo_path, commit_msg);
        }
        Err(e) => panic!("Could not open editor at path {}: {}", editor_path, e),
    };
}

fn display_first_time_setup_banner() {
    println!();
    println!("##########################################################");
    println!("####                 First Time Setup                 ####");
    println!("##########################################################");
    println!();
    println!("This tool requires you to have a repository with the a");
    println!("README.md in the root folder. The markdown file is where");
    println!("your ideas will be stored.");
    println!();
}

fn get_commit_msg() -> String {
    println!("Idea commit subject: ");
    let mut input = String::new();
    // The library text_io doesn't read input
    // if it has any whitespace in it
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn open_editor(bin_path: &String, file_path: &String) -> Result<(), Error> {
    match Command::new(bin_path).arg(file_path).status() {
        Ok(_) => Ok(()),
        Err(e) => {
            println!(
                "Unable to open file [{}] with editor binary at [{}]: {}",
                file_path, bin_path, e
            );
            Err(e)
        }
    }
}

/*
 * Borrow helpers
*/

fn s(string: &str) -> String {
    string.to_owned()
}
