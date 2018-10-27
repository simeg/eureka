pub mod git {
    use std::io::Result;
    use std::process::Command;
    use utils::utils;

    pub fn git_commit_and_push(repo_path: &String, msg: String) -> Result<()> {
        git_add(repo_path, &msg)
            .and(git_commit(repo_path, msg))
            .and(git_push(repo_path))
    }

    fn git_add(repo_path: &String, commit_msg: &String) -> Result<()> {
        match Command::new(git())
            .args(default_args(repo_path).iter())
            .arg("add")
            .arg("./README.md")
            .arg(format!("./ideas/{}.md", utils::format_idea_filename(commit_msg)))
            .status()
        {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Could not stage files to repo at [{}]: {}", repo_path, e);
                Err(e)
            }
        }
    }

    fn git_commit(repo_path: &String, msg: String) -> Result<()> {
        match Command::new(git())
            .args(default_args(repo_path).iter())
            .arg("commit")
            .arg("-m")
            .arg(msg)
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

    pub fn get_repo_url(repo_path: &String) -> Result<Vec<u8>> {
        Ok(Command::new(git())
               .args(default_args(repo_path).iter())
               .arg("config")
               .arg("--get")
               .arg("remote.origin.url")
               .output()
               .expect("Could not get remote url.")
               .stdout)
    }

    fn git() -> String {
        if utils::is_program_in_path("git") {
            String::from("git")
        } else {
            panic!("Cannot locate executable - git - on your system")
        }
    }

    fn default_args(repo_path: &String) -> [String; 2] {
        [
            format!("--git-dir={}/.git/", repo_path),
            format!("--work-tree={}", repo_path),
        ]
    }
}
