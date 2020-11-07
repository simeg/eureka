#[cfg(test)]
extern crate atomic_counter;
extern crate dirs;
extern crate git2;
extern crate termcolor;

use std::io;

use crate::git::GitManagement;
use crate::program_access::ProgramOpener;
use file_handler::{ConfigManagement, FileManagement};
use printer::{Print, PrintColor};
use reader::ReadInput;
use std::io::{Error, ErrorKind};
use types::ConfigFile::{Branch, Repo};

pub mod types;

pub mod file_handler;
pub mod git;
pub mod printer;
pub mod program_access;
pub mod reader;

pub struct Eureka<
    FH: ConfigManagement + FileManagement,
    W: Print + PrintColor,
    R: ReadInput,
    G: GitManagement,
    PO: ProgramOpener,
> {
    fh: FH,
    printer: W,
    reader: R,
    git: G,
    program_opener: PO,
}

pub struct EurekaOptions {
    pub clear_repo: bool,
    pub clear_branch: bool,
    pub view: bool,
}

impl<FH, W, R, G, PO> Eureka<FH, W, R, G, PO>
where
    FH: ConfigManagement + FileManagement,
    W: Print + PrintColor,
    R: ReadInput,
    G: GitManagement,
    PO: ProgramOpener,
{
    pub fn new(fh: FH, printer: W, reader: R, git: G, program_opener: PO) -> Self {
        Eureka {
            fh,
            printer,
            reader,
            git,
            program_opener,
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
            self.open_idea_file()?;
            return Ok(());
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
            Ok(())
        } else {
            self.ask_for_idea()
        }
    }

    fn ask_for_idea(&mut self) -> io::Result<()> {
        // TODO: Ask again if empty input
        self.printer.input_header(">> Idea summary");
        let idea_summary = self.reader.read_input();

        let repo_path = self.fh.config_read(Repo)?;
        self.git
            .init(&repo_path)
            .map_err(|git_err| Error::new(ErrorKind::InvalidInput, git_err))?;

        self.program_opener
            .open_editor(&format!("{}/README.md", repo_path))
            .and(self.git_add_commit_push(idea_summary))
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
        self.program_opener
            .open_pager(&format!("{}/README.md", repo_path))
    }

    fn git_add_commit_push(&mut self, commit_subject: String) -> io::Result<()> {
        self.printer
            .println("Adding and committing your new idea..");
        let branch_name = self
            .fh
            .config_read(Branch)
            .unwrap_or_else(|_| panic!("Branch config is missing (should never end up here"));
        self.git.checkout_branch(&*branch_name)
            .expect("Something went wrong checking out branch");
        self.git.add()
            .and_then(|_| self.git.commit(commit_subject))
        let branch_name = self
            .fh
            .config_read(Branch)
            .unwrap_or_else(|_| panic!("Branch config is missing (should never end up here"));
        git.checkout_branch(&*branch_name)
            .expect("Something went wrong checking out branch");
        git.add()
            .and_then(|_| git.commit(commit_subject))
            .expect("Something went wrong adding or committing");
        self.printer.println("Added and committed!");

        self.printer.println("Pushing your new idea..");
        self.git.push(&*branch_name)
            .expect("Something went wrong pushing");
        self.git.push().expect("Something went wrong pushing");
        git.push(&*branch_name)
            .expect("Something went wrong pushing");
        self.printer.println("Pushed!");

        Ok(())
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

        match self.program_opener.open_editor(&readme_path) {
            Ok(_) => self.git_add_commit_push(idea_summary),
            Err(e) => panic!(e),
        };
    }
}
