use git2::{
    Commit, Error, ErrorClass, ErrorCode, ObjectType, Oid, PushOptions, RemoteCallbacks, Repository,
};

use std::path::Path;

pub trait GitManagement {
    fn init(&mut self, repo_path: &str) -> Result<(), git2::Error>;
    fn checkout_branch(&self, branch_name: &str) -> Result<(), git2::Error>;
    fn add(&self) -> Result<(), git2::Error>;
    fn commit(&self, subject: &str) -> Result<Oid, git2::Error>;
    fn push(&self, branch_name: &str) -> Result<(), git2::Error>;
}

pub struct Git {
    repo: Option<Repository>,
}

impl Default for Git {
    fn default() -> Self {
        Self { repo: None }
    }
}

impl GitManagement for Git {
    fn init(&mut self, repo_path: &str) -> Result<(), git2::Error> {
        Repository::open(&Path::new(&repo_path)).map(|repo| self.repo = Some(repo))
    }

    fn checkout_branch(&self, branch_name: &str) -> Result<(), git2::Error> {
        let repo = self.repo.as_ref().unwrap();

        let commit = repo
            .head()
            .map(|head| head.target())
            .and_then(|oid| repo.find_commit(oid.unwrap()))?;

        // Create new branch if it doesn't exist
        match repo.branch(branch_name, &commit, false) {
            // This command can fail due to an existing reference. This error should be ignored.
            Err(err)
                if !(err.class() == ErrorClass::Reference && err.code() == ErrorCode::Exists) =>
            {
                return Err(err);
            }
            _ => {}
        }

        let refname = format!("refs/heads/{}", branch_name);
        let obj = repo.revparse_single(&*refname)?;

        repo.checkout_tree(&obj, None)?;

        repo.set_head(&*refname)
    }

    fn add(&self) -> Result<(), git2::Error> {
        let mut index = self.repo.as_ref().unwrap().index()?;

        index.add_path(Path::new("README.md"))?;
        index.write()
    }

    fn commit(&self, subject: &str) -> Result<Oid, git2::Error> {
        let repo = self.repo.as_ref().unwrap();
        let mut index = repo.index()?;

        let signature = repo.signature()?; // Use default user.name and user.email

        let oid = index.write_tree()?;
        let parent_commit = self.find_last_commit()?;
        let tree = repo.find_tree(oid)?;

        repo.commit(
            Some("HEAD"),      // point HEAD to our new commit
            &signature,        // author
            &signature,        // committer
            &subject,          // commit message
            &tree,             // tree
            &[&parent_commit], // parent commit
        )
    }

    fn push(&self, branch_name: &str) -> Result<(), git2::Error> {
        self.push(&branch_name, self.get_callbacks())
    }
}

const GIT_USERNAME_DEFAULT: &str = "git";

impl Git {
    fn find_last_commit(&self) -> Result<Commit, git2::Error> {
        let obj = self
            .repo
            .as_ref()
            .unwrap()
            .head()?
            .resolve()?
            .peel(ObjectType::Commit)?;
        obj.into_commit()
            .map_err(|_| git2::Error::from_str("Couldn't find commit"))
    }
}

impl Git {
    fn get_callbacks(&self) -> RemoteCallbacks {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|url, username_from_url, allowed_types| {
            let username = username_from_url.unwrap_or(GIT_USERNAME_DEFAULT);

            debug!("get_callbacks: got username: {}", &username);
            debug!("get_callbacks: allowed types: {:?}", &allowed_types);

            if allowed_types.contains(git2::CredentialType::SSH_KEY)
                || allowed_types.contains(git2::CredentialType::SSH_CUSTOM)
                || allowed_types.contains(git2::CredentialType::SSH_MEMORY)
            {
                debug!("get_callbacks: allowed types contains ssh custom or ssh key");
                let result = git2::Cred::ssh_key_from_agent(username);

                if result.is_err() {
                    debug!(
                        "get_callbacks: got auth error: {}",
                        result.as_ref().err().unwrap()
                    );
                } else {
                    debug!("get_callbacks: got creds");
                    let cred = result.as_ref().unwrap();
                    debug!("get_callbacks: cred username: {}", cred.has_username());
                    debug!("get_callbacks: cred type: {}", cred.credtype());
                }

                result
            } else if allowed_types.contains(git2::CredentialType::USERNAME) {
                debug!("get_callbacks: allowed_types contains username");
                let result = git2::Cred::username(username);

                if result.is_err() {
                    debug!(
                        "get_callbacks: got auth error: {}",
                        result.as_ref().err().unwrap()
                    );
                }

                result
            } else {
                debug!("get_callbacks: using credential helper");
                let config = git2::Config::open_default()?;
                let result = git2::Cred::credential_helper(&config, url, Some(username));

                if result.is_err() {
                    debug!(
                        "get_callbacks: got auth error: {}",
                        result.as_ref().err().unwrap()
                    );
                } else {
                    debug!("get_callbacks: got creds");
                    let cred = result.as_ref().unwrap();
                    debug!("get_callbacks: cred username: {}", cred.has_username());
                    debug!("get_callbacks: cred type: {}", cred.credtype());
                }

                result
            }
        });
        callbacks
    }

    fn push(&self, branch_name: &&str, callbacks: RemoteCallbacks) -> Result<(), Error> {
        let mut remote = self.repo.as_ref().unwrap().find_remote("origin")?;

        let mut options = PushOptions::new();
        options.remote_callbacks(callbacks);

        let result = remote.push(
            &[format!(
                "refs/heads/{}:refs/heads/{}",
                branch_name, branch_name
            )],
            Some(&mut options),
        );

        if result.is_err() {
            debug!(
                "push: could not push due to error: {}",
                result.as_ref().err().unwrap()
            );
        }

        result
    }
}
