extern crate dirs;
extern crate git2;
extern crate termcolor;

use std::process::Command;
use std::{env, io};

use file_handler::{ConfigManagement, FileManagement};
use git::Git;
use printer::{Print, PrintColor};
use reader::ReadInput;
use types::ConfigFile::{Branch, Repo};
use utils::get_if_available;

pub mod types;
pub mod utils;

pub mod file_handler;
mod git;
pub mod printer;
pub mod reader;

pub struct Eureka<FH: ConfigManagement + FileManagement, W: Print + PrintColor, R: ReadInput> {
    fh: FH,
    printer: W,
    reader: R,
    git: Option<Git>,
}

pub struct EurekaOptions {
    pub clear_repo: bool,
    pub clear_branch: bool,
    pub view: bool,
}

impl<FH, W, R> Eureka<FH, W, R>
where
    FH: ConfigManagement + FileManagement,
    W: Print + PrintColor,
    R: ReadInput,
{
    pub fn new(fh: FH, printer: W, reader: R) -> Self {
        Eureka {
            fh,
            printer,
            reader,
            git: None,
        }
    }

    pub fn run(&mut self, opts: EurekaOptions) -> io::Result<()> {
        if opts.clear_repo || opts.clear_branch {
            if opts.clear_repo {
                self.clear_repo()?;
            }

            if opts.clear_branch {
                self.clear_branch()?;
            }

            return Ok(());
        }

        if opts.view {
            self.open_idea_file()?
        }

        if self.is_config_missing() {
            // If config dir is missing - create it
            if !self.fh.config_dir_exists() {
                self.fh.config_dir_create()?;
            }

            self.printer.fts_banner();

            // If repo path is missing - ask for it
            if self.fh.config_read(Repo).is_err() {
                self.setup_repo_path()?;
            }

            // If branch name is missing - ask for it
            if self.fh.config_read(Branch).is_err() {
                self.setup_branch_name()?;
            }

            self.printer
                .print("First time setup complete. Happy ideation!");
        } else {
            self.ask_for_idea();
        }

        Ok(())
    }

    fn clear_repo(&self) -> io::Result<()> {
        self.fh
            .config_read(Repo)
            .and_then(|_| self.fh.file_rm(Repo))
    }

    fn clear_branch(&self) -> io::Result<()> {
        self.fh
            .config_read(Branch)
            .and_then(|_| self.fh.file_rm(Branch))
    }

    fn open_idea_file(&self) -> io::Result<()> {
        let repo_path = self.fh.config_read(Repo)?;
        self.open_pager(repo_path)
    }

    fn init_git(&mut self) {
        let repo_path = self
            .fh
            .config_read(Repo)
            .unwrap_or_else(|_| panic!("Repo config is missing (should never end up here"));
        self.git = Some(Git::new(repo_path));
    }

    fn git_add_commit_push(&mut self, commit_subject: String) {
        let git = self.git.as_ref().unwrap();

        self.printer
            .println("Adding and committing your new idea..");
        git.add()
            .and_then(|_| git.commit(commit_subject))
            .expect("Something went wrong adding or committing");
        self.printer.println("Added and committed!");

        self.printer.println("Pushing your new idea..");

        let branch_name = self.fh.config_read(Branch).unwrap();
        git.push(branch_name).expect("Something went wrong pushing");
        self.printer.println("Pushed!");
    }

    fn setup_repo_path(&mut self) -> io::Result<()> {
        let mut input_repo_path = String::new();

        while input_repo_path.is_empty() {
            self.printer.input_header("Absolute path to your idea repo");
            input_repo_path = self.reader.read_input();
        }

        self.fh.config_write(Repo, input_repo_path)
    }

    fn setup_branch_name(&mut self) -> io::Result<()> {
        self.printer
            .input_header("Name of branch (default: master)");
        let mut branch_name = self.reader.read_input();

        // Default to "master"
        if branch_name.is_empty() {
            branch_name = "master".to_string();
        }

        self.fh.config_write(Branch, branch_name)
    }

    fn is_config_missing(&self) -> bool {
        self.fh.config_read(Repo).is_err() || self.fh.config_read(Branch).is_err()
    }

    fn ask_for_idea(&mut self) {
        // TODO: Ask again if empty input
        self.printer.input_header(">> Idea summary");
        let idea_summary = self.reader.read_input();

        let repo_path = self.fh.config_read(Repo).unwrap();
        let readme_path = format!("{}/README.md", repo_path);

        self.init_git();

        match self.open_editor(&readme_path) {
            Ok(_) => self.git_add_commit_push(idea_summary),
            Err(e) => panic!(e),
        };
    }

    fn open_editor(&self, file_path: &str) -> io::Result<()> {
        let editor = match env::var("EDITOR") {
            Ok(e) => e,
            Err(_) => {
                get_if_available("vi").expect("Cannot locate executable - vi - on your system")
            }
        };
        match Command::new(&editor).arg(file_path).status() {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!(
                    "Error: Unable to open file [{}] with editor binary at [{}]: {}",
                    file_path, editor, e
                );
                Err(e)
            }
        }
    }

    fn open_pager(&self, repo_config_file: String) -> io::Result<()> {
        let readme_path = format!("{}/README.md", repo_config_file);
        let pager = match env::var("PAGER") {
            Ok(p) => p,
            Err(_) => {
                get_if_available("less").expect("Cannot locate executable - less - on your system")
            }
        };
        match Command::new(&pager).arg(&readme_path).status() {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!(
                    "Error: Could not open idea file with {} at [{}]: {}",
                    pager, readme_path, e
                );
                Err(e)
            }
        }
    }
}
