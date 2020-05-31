use std::io::Result;
use std::process::Command;
use utils;

pub fn commit_and_push(repo_path: &str, subject: String) -> Result<()> {
    add(repo_path)
        .and(commit(repo_path, subject))
        .and(push(repo_path))
}

fn add(repo_path: &str) -> Result<()> {
    match Command::new(git())
        .args(default_args(repo_path).iter())
        .arg("add")
        .arg("./README.md")
        .status()
    {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Could not stage files to repo at [{}]: {}", repo_path, e);
            Err(e)
        }
    }
}

fn commit(repo_path: &str, subject: String) -> Result<()> {
    match Command::new(git())
        .args(default_args(repo_path).iter())
        .arg("commit")
        .arg("-m")
        .arg(subject)
        .status()
    {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!(
                "Could not commit new idea to repo at [{}]: {}",
                repo_path, e
            );
            Err(e)
        }
    }
}

fn push(repo_path: &str) -> Result<()> {
    match Command::new(git())
        .args(default_args(repo_path).iter())
        .arg("push")
        .arg("origin")
        .arg("master")
        .status()
    {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!(
                "Could not push commit to remote 'origin' and \
                     branch 'master' in repo at [{}]: {}",
                repo_path, e
            );
            Err(e)
        }
    }
}

fn git() -> String {
    utils::get_if_available("git").expect("Cannot locate executable - git - on your system")
}

fn default_args(repo_path: &str) -> [String; 2] {
    [
        format!("--git-dir={}/.git/", repo_path),
        format!("--work-tree={}", repo_path),
    ]
}
