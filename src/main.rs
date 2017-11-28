use std::env;
use std::io;
use std::fs;
use std::io::{Read, Write};
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
    let _cli_flags: ArgMatches = App::new("idea")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Quickly save your ideas without leaving the terminal")
        .arg(Arg::with_name("repo-path")
            .short("r")
            .long("path")
            .takes_value(true)
            .help("Absolute path to the repo where you have the README.md with ideas"))
        .arg(Arg::with_name("default-commit-msg")
            .short("m")
            .long("msg")
            .takes_value(true)
            .help("The git commit message used if you don't specify one"))
        .get_matches();

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

            // TODO: Handle if extra / on the end

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

    let readme_path: String = format!("{}/README.md", repo_path);
    match open_editor(&editor_path, &readme_path) {
        Ok(_) => {
            let git_result = git_commit_and_push(&repo_path, s(""));
        }
        Err(_) => {}
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

fn open_editor(bin_path: &String, file_path: &String) -> Result<(), ()> {
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
            Err(())
        }
    }
}


/*
 * Git
*/

fn git_commit_and_push(repo_path: &String, msg: String) -> Result<(), ()> {
    let git = get_git_path().unwrap(); // TODO

    // Use the results
    let _git_add = git_add(repo_path);
    let _git_commit = git_commit(repo_path, String::from("This is a test commit, drop me if you see me"));

    Ok(())
}

fn get_git_path() -> Result<String, ()> {
    // TODO: Do not have it hard-coded, look for it in common places
    Ok(String::from("/usr/bin/git"))
}

fn git_add(repo_path: &String) -> Result<(), ()> {
    let git = get_git_path().unwrap(); // TODO
    match Command::new(git)
        .arg(format!("--git-dir={}/.git/", repo_path))
        .arg(format!("--work-tree={}", repo_path))
        .arg("add")
        .arg("-A")
        .status() {
        Ok(_) => Ok(()),
        Err(e) => {
            panic!("Could not add files with git in repo at [{}]: {}", repo_path, e);
        }
    }
}

fn git_commit(repo_path: &String, msg: String) -> Result<(), ()> {
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
            println!("Could not commit new idea with git in repo at [{}]: {}", repo_path, e);
            Err(())
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
        Some(location) => format!("{}/{}", location.display(), ".idea-cli/config"),
        None => panic!("Could not resolve your $HOME directory")
    }
}


/*
 * Borrow helpers
*/

fn s(string: &str) -> String {
    string.to_owned()
}
