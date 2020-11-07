use git2::{Commit, Cred, Direction, ObjectType, Oid, PushOptions, RemoteCallbacks, Repository};

use std::path::Path;

pub struct Git {
    repo: Repository,
}

impl Git {
    const IDEA_FILE_NAME: &'static str = "README.md";

    pub fn new(repo_path: String) -> Self {
        let repo = Repository::open(&Path::new(&repo_path))
            .unwrap_or_else(|_| panic!("Could not locate repo at: {}", repo_path));
        Self { repo }
    }

    pub fn checkout_branch(&self, branch_name: &str) -> Result<(), git2::Error> {
        let repo = &self.repo;

        let commit = repo
            .head()
            .map(|head| head.target())
            .and_then(|oid| repo.find_commit(oid.unwrap()))?;

        // Create new branch if it doesn't exist and swallow error
        let _branch = repo.branch(branch_name, &commit, false);

        let refname = format!("refs/heads/{}", branch_name);
        let obj = repo.revparse_single(&*refname)?;

        repo.checkout_tree(&obj, None)?;

        repo.set_head(&*refname)
    }

    pub fn add(&self) -> Result<(), git2::Error> {
        let mut index = self.repo.index()?;

        index.add_path(Path::new(Git::IDEA_FILE_NAME))?;
        index.write()
    }

    pub fn commit(&self, subject: String) -> Result<Oid, git2::Error> {
        let mut index = self.repo.index()?;

        let signature = self.repo.signature()?; // Use default user.name and user.email

        let oid = index.write_tree()?;
        let parent_commit = self.find_last_commit()?;
        let tree = self.repo.find_tree(oid)?;

        self.repo.commit(
            Some("HEAD"),      // point HEAD to our new commit
            &signature,        // author
            &signature,        // committer
            &subject,          // commit message
            &tree,             // tree
            &[&parent_commit], // parent commit
        )
    }

    pub fn push(&self, branch_name: &str) -> Result<(), git2::Error> {
        let mut remote = self.repo.find_remote("origin").unwrap();

        remote.connect_auth(Direction::Push, Some(self.get_callbacks()), None)?;

        let mut options = PushOptions::default();
        options.remote_callbacks(self.get_callbacks());
        remote.push(
            &[format!(
                "refs/heads/{}:refs/heads/{}",
                branch_name, branch_name
            )],
            Some(&mut options),
        )
    }

    fn find_last_commit(&self) -> Result<Commit, git2::Error> {
        let obj = self.repo.head()?.resolve()?.peel(ObjectType::Commit)?;
        obj.into_commit()
            .map_err(|_| git2::Error::from_str("Couldn't find commit"))
    }

    fn get_callbacks<'a>(&self) -> RemoteCallbacks<'a> {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username, _allowed_types| {
            Cred::ssh_key_from_agent(username.unwrap())
        });
        callbacks
    }
}
