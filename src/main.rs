use std::env;
use std::fs;
use std::path::Path;
use std::io::{Read, Write};
use std::process::{Command, ExitStatus};

#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
extern crate serde;

#[macro_use]
extern crate clap;
use clap::{ArgMatches};

use clap::{App, Arg};

fn main() {
    let cli_flags: ArgMatches = App::new("idea")
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

    // Ask what editor to use
    // - vim
    // - nano
    // - ed
    // - what more?
    // (Use fallback editor)

    //    let user_inputted_repo_path = String::from("/Users/simon/repos/ideas"); // TODO: Get from user + change var name
    //    let user_inputted_editor_path = String::from("/usr/bin/vim"); // TODO: Get from user + change var name

    let repo_path: String = match read_from_config(s("repo_path")) {
        Some(file_path) => file_path,
        None => panic!("Could not read repo path file"),
    };

    let preferred_editor: String = match read_from_config(s("preferred_editor_path")) {
        Some(editor_bin_path) => editor_bin_path,
        None => panic!("Could not read preferred editor path file"),
    };

    let readme_path: String = format!("{}/README.md", repo_path);
    match open_editor(&preferred_editor, &readme_path) {
        Ok(_) => {
            let git_result = git_commit_and_push(&repo_path, s(""));
        }
        Err(_) => {}
    };
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
            Err(())
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
 * File utils
*/

fn file_exists(path: &String) -> bool {
    match ::fs::File::open(&path) {
        Ok(_) => true,
        Err(_) => false
    }
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

fn write_to_config<T: ::serde::Serialize>(key: &str, data: T) -> Result<T, ()> {
    let location = get_config_location();
    let path = format!("{}/{}", location, key);
    match ::fs::File::create(path) {
        Ok(mut file) => {
            match ::json::to_string::<T>(&data) {
                Ok(str_data) => {
                    let _ = file.write(&str_data.into_bytes());
                    Ok(data)
                }
                Err(e) => panic!("Could not deserialize data: {}", e)
            }
        }
        Err(_) => {
            // TODO: Overwrite existing value
            println!("File for for [{}] already exist, doing nothing", key);
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
