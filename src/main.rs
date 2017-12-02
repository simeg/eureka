#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use std::env;
use std::io;
use std::fs;
use std::io::{Read, Write, Error};
use std::process::Command;

extern crate serde_json as json;
extern crate serde;


#[macro_use]
extern crate text_io;

#[macro_use]
extern crate clap;

use clap::ArgMatches;
use clap::{App, Arg};


fn main() {
    let cli_flags: ArgMatches = App::new("eureka")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Input and store your ideas without leaving the terminal")
        .arg(Arg::with_name("clear-repo")
            .long("clear-repo")
            .help("Use this flag to clear the stored path to your idea repo"))
        .arg(Arg::with_name("clear-editor")
            .long("clear-editor")
            .help("Use this flag to clear the stored path to your idea editor"))
        .get_matches();

    if cli_flags.is_present("clear-repo") {
        // Clear repo config value
    }

    if cli_flags.is_present("clear-repo") {
        // Clear repo config value
    }
    // TODO: Add param for clearing saved repo/editor

    let repo_path: String = match read_from_config(s("repo_path")) {
        Some(file_path) => file_path,
        None => {
            display_first_time_setup_banner();
            if !path_exists(&get_config_location()) {
                match fs::create_dir_all(&get_config_location()) {
                    Ok(_) => {}
                    Err(_) => panic!(
                        "Could not create dir at {} to store necessary config",
                        get_config_location()
                    ),
                }
            }

            print!("Absolute path to your idea repo: ");
            io::stdout().flush().unwrap();
            let input_path: String = read!();
            let copy_input_path: String = input_path.clone();

            match write_to_config("repo_path", input_path) {
                Ok(_) => copy_input_path,
                Err(e) => panic!("Unable to write your repo path to disk: {}", e),
            }
        }
    };

    let editor_path: String = match read_from_config(s("editor_path")) {
        Some(file_path) => file_path,
        None => {
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

            if !path_exists(&input_path) {
                panic!("Invalid editor path");
            }

            let copy_input_path: String = input_path.clone();
            match write_to_config("editor_path", input_path) {
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
    match Command::new(bin_path)
        .arg(file_path)
        .status() {
        Ok(_) => Ok(()),
        Err(e) => {
            println!(
                "Unable to open file [{}] with editor binary at [{}]: {}",
                file_path,
                bin_path,
                e
            );
            Err(e)
        }
    }
}


/*
 * Git
*/

fn git_commit_and_push(repo_path: &String, msg: String) -> Result<(), ()> {
    // TODO: See how to chain these function calls
    git_add(repo_path).unwrap();
    git_commit(repo_path, msg).unwrap();
    git_push(repo_path).unwrap();
    Ok(())
}

fn get_git_path() -> Result<String, ()> {
    // TODO: Do not have it hard-coded, look for it in common places
    Ok(String::from("/usr/bin/git"))
}

fn git_add(repo_path: &String) -> Result<(), Error> {
    let git = get_git_path().unwrap(); // TODO
    match Command::new(git)
        .arg(format!("--git-dir={}/.git/", repo_path))
        .arg(format!("--work-tree={}", repo_path))
        .arg("add")
        .arg("-A")
        .status() {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Could not stage files to repo at [{}]: {}", repo_path, e);
            Err(e)
        }
    }
}

fn git_commit(repo_path: &String, msg: String) -> Result<(), Error> {
    let git = get_git_path().unwrap(); // TODO
    match Command::new(git)
        .arg(format!("--git-dir={}/.git/", repo_path))
        .arg(format!("--work-tree={}", repo_path))
        .arg("commit")
        .arg("-m")
        .arg(msg)
        .status() {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Could not commit new idea to repo at [{}]: {}", repo_path, e);
            Err(e)
        }
    }
}

fn git_push(repo_path: &String) -> Result<(), Error> {
    let git = get_git_path().unwrap(); // TODO
    match Command::new(git)
        .arg(format!("--git-dir={}/.git/", repo_path))
        .arg(format!("--work-tree={}", repo_path))
        .arg("push")
        .arg("origin")
        .arg("master")
        .status() {
        Ok(_) => Ok(()),
        Err(e) => {
            println!(
                "Could not push commit to remote 'origin' and \
                branch 'master' in repo at [{}]: {}",
                repo_path,
                e
            );
            Err(e)
        }
    }
}


/*
 * File and folder utils
*/

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn read_from_config<T: ::serde::Deserialize>(key: String) -> Option<T> {
    let location = get_config_location();
    let path = format!("{}/{}", location, key);
    match ::fs::File::open(&path) {
        Ok(mut file) => {
            let mut raw = String::new();
            match file.read_to_string(&mut raw) {
                Ok(_) => match ::json::from_str::<T>(&raw) {
                    Ok(res) => Some(res),
                    Err(e) => panic!(
                        "Unable to serialize [{}] from JSON with error: {}",
                        key,
                        e
                    )
                },
                Err(_) => None
            }
        }
        Err(_) => None
    }
}

fn write_to_config<T: ::serde::Serialize>(key: &str, data: T) -> Result<T, (json::Error)> {
    let location = get_config_location();
    let path = format!("{}/{}", location, key);
    match ::fs::File::create(path) {
        Ok(mut file) => {
            match ::json::to_string::<T>(&data) {
                Ok(str_data) => {
                    let _ = file.write(&str_data.into_bytes());
                    Ok(data)
                }
                Err(e) => Err(e)
            }
        }
        Err(_) => {
            // TODO: Overwrite existing value, use additional param to decide it
            // File for [key] already exist, doing nothing
            Ok(data)
        }
    }
}

fn get_config_location() -> String {
    match ::env::home_dir() {
        Some(location) => format!("{}/{}", location.display(), ".eureka/"),
        None => panic!("Could not resolve your $HOME directory")
    }
}


/*
 * Borrow helpers
*/

fn s(string: &str) -> String {
    string.to_owned()
}
