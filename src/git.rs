pub mod git {
    use std::io::Result;
    use std::process::Command;
    use utils::utils;

    pub fn git_commit_and_push(repo_path: &String, subject: String) -> Result<()> {
        git_add(repo_path)
            .and(git_commit(repo_path, subject))
            .and(git_push(repo_path))
    }

    fn git_add(repo_path: &String) -> Result<()> {
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

    fn git_commit(repo_path: &String, subject: String) -> Result<()> {
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

    fn git_push(repo_path: &String) -> Result<()> {
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

    fn default_args(repo_path: &String) -> [String; 2] {
        [
            format!("--git-dir={}/.git/", repo_path),
            format!("--work-tree={}", repo_path),
        ]
    }
}
