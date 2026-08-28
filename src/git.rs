use std::fs;
use std::path::{Path, PathBuf};

pub trait GitManagement {
    fn init(&mut self, repo_path: &str) -> Result<(), git2::Error>;
    fn checkout_branch(&self, branch_name: &str) -> Result<(), git2::Error>;
    fn add(&self, filename: &str) -> Result<(), git2::Error>;
    fn commit(&self, subject: &str) -> Result<git2::Oid, git2::Error>;
    fn push(&self, branch_name: &str) -> Result<(), git2::Error>;
}

#[derive(Default)]
pub struct Git {
    repo: Option<git2::Repository>,
}

impl GitManagement for Git {
    fn init(&mut self, repo_path: &str) -> Result<(), git2::Error> {
        git2::Repository::open(Path::new(&repo_path)).map(|repo| self.repo = Some(repo))
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
                if !(err.class() == git2::ErrorClass::Reference
                    && err.code() == git2::ErrorCode::Exists) =>
            {
                return Err(err);
            }
            _ => {}
        }

        let refname = format!("refs/heads/{}", branch_name);
        let obj = repo.revparse_single(refname.as_str())?;

        repo.checkout_tree(&obj, None)?;
        repo.set_head(refname.as_str())
    }

    fn add(&self, filename: &str) -> Result<(), git2::Error> {
        let mut index = self.repo.as_ref().unwrap().index()?;

        index.add_path(Path::new(filename))?;
        index.write()
    }

    fn commit(&self, subject: &str) -> Result<git2::Oid, git2::Error> {
        let repo = self.repo.as_ref().unwrap();
        let mut index = repo.index()?;

        let signature = repo.signature()?; // Use default user.name and user.email

        let oid = index.write_tree()?;
        let parent_commit = find_last_commit(self.repo.as_ref().unwrap())?;
        let tree = repo.find_tree(oid)?;

        repo.commit(
            Some("HEAD"),      // point HEAD to our new commit
            &signature,        // author
            &signature,        // committer
            subject,           // commit message
            &tree,             // tree
            &[&parent_commit], // parent commit
        )
    }

    fn push(&self, branch_name: &str) -> Result<(), git2::Error> {
        with_credentials(self.repo.as_ref().unwrap(), |cred_callback| {
            let mut remote = self.repo.as_ref().unwrap().find_remote("origin")?;

            let mut callbacks = git2::RemoteCallbacks::new();
            let mut options = git2::PushOptions::new();

            callbacks.credentials(cred_callback);
            options.remote_callbacks(callbacks);

            remote.push(
                &[format!(
                    "refs/heads/{}:refs/heads/{}",
                    branch_name, branch_name
                )],
                Some(&mut options),
            )?;

            Ok(())
        })
    }
}

fn find_last_commit(repo: &git2::Repository) -> Result<git2::Commit<'_>, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(git2::ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

/// Private key filenames to try, in priority order.
const SSH_KEY_NAMES: [&str; 2] = ["id_ed25519", "id_rsa"];

/// Resolve usable SSH key paths from `~/.ssh`, in priority order.
fn ssh_key_paths() -> Vec<(PathBuf, Option<PathBuf>)> {
    match dirs::home_dir() {
        Some(home) => ssh_key_paths_in(&home.join(".ssh")),
        None => vec![],
    }
}

/// Resolve usable SSH key paths from `ssh_dir`, in priority order.
///
/// Only keys we can actually authenticate with are returned. libgit2 aborts the
/// entire credential chain (rather than advancing to the next method) when
/// libssh2 fails to *load* a key, so an unreadable or passphrase-protected key
/// left in this list would block the credential-helper and default fallbacks.
fn ssh_key_paths_in(ssh_dir: &Path) -> Vec<(PathBuf, Option<PathBuf>)> {
    SSH_KEY_NAMES
        .iter()
        .filter_map(|name| {
            let private = ssh_dir.join(name);
            let contents = fs::read(&private).ok()?;
            if is_passphrase_protected(&contents) {
                return None;
            }

            let public = ssh_dir.join(format!("{}.pub", name));
            let public = public.is_file().then_some(public);
            Some((private, public))
        })
        .collect()
}

/// File magic identifying the modern OpenSSH private key format.
const OPENSSH_MAGIC: &[u8] = b"openssh-key-v1\0";

/// Detect an encrypted private key, which we cannot use without a passphrase.
///
/// Covers both the legacy PEM header and the modern OpenSSH format, where the
/// cipher name follows the magic and is `none` when the key is unencrypted.
fn is_passphrase_protected(key: &[u8]) -> bool {
    if key.starts_with(OPENSSH_MAGIC) {
        // Binary format: magic, then a length-prefixed cipher name.
        let rest = &key[OPENSSH_MAGIC.len()..];
        let Some((len, rest)) = rest.split_first_chunk::<4>() else {
            return true; // Malformed - treat as unusable
        };
        let len = u32::from_be_bytes(*len) as usize;
        return rest.get(..len) != Some(b"none");
    }

    // Base64-armoured OpenSSH keys and legacy PEM keys are both text.
    let text = String::from_utf8_lossy(key);
    if text.contains("Proc-Type: 4,ENCRYPTED") || text.contains("ENCRYPTED PRIVATE KEY") {
        return true;
    }

    // Armoured OpenSSH: decode enough of the body to read the cipher name.
    if text.contains("BEGIN OPENSSH PRIVATE KEY") {
        let body: String = text
            .lines()
            .filter(|line| !line.starts_with("-----"))
            .collect();
        return match base64_decode(&body) {
            Some(decoded) => is_passphrase_protected(&decoded),
            None => true,
        };
    }

    false
}

/// Decode base64 `input`, ignoring any trailing padding.
fn base64_decode(input: &str) -> Option<Vec<u8>> {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let symbols: Vec<u8> = input
        .bytes()
        .take_while(|b| *b != b'=')
        .map(|b| ALPHABET.iter().position(|a| *a == b).map(|i| i as u8))
        .collect::<Option<Vec<u8>>>()?;

    // Each base64 symbol carries 6 bits, so every full 8 bits form a byte.
    let mut out = Vec::with_capacity(symbols.len() * 6 / 8);
    let mut buffer = 0u32;
    let mut bits = 0u32;
    for symbol in symbols {
        buffer = (buffer << 6) | u32::from(symbol);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buffer >> bits) as u8);
        }
    }

    Some(out)
}

/// Helper to run git operations that require authentication.
///
/// This is inspired by [the way Cargo handles this][cargo-impl].
///
/// [cargo-impl]: https://github.com/rust-lang/cargo/blob/94bf4781d0bbd266abe966c6fe1512bb1725d368/src/cargo/sources/git/utils.rs#L437
fn with_credentials<F>(repo: &git2::Repository, mut f: F) -> Result<(), git2::Error>
where
    F: FnMut(&mut git2::Credentials) -> Result<(), git2::Error>,
{
    let config = repo.config()?;
    let ssh_keys = ssh_key_paths();

    let mut tried_sshagent = false;
    let mut ssh_key_idx = 0;
    let mut tried_cred_helper = false;
    let mut tried_default = false;

    f(&mut |url, username, allowed| {
        if allowed.contains(git2::CredentialType::USERNAME) {
            return Err(git2::Error::from_str("No username specified in remote URL"));
        }

        // 1. Try ssh-agent
        if allowed.contains(git2::CredentialType::SSH_KEY) && !tried_sshagent {
            tried_sshagent = true;
            let username = username.unwrap();
            return git2::Cred::ssh_key_from_agent(username);
        }

        // 2. Try SSH keys from disk (~/.ssh/id_ed25519, ~/.ssh/id_rsa).
        // Only keys ssh_key_paths deemed usable reach this point, so a failure
        // here is a genuine rejection and libgit2 will retry with the next one.
        if allowed.contains(git2::CredentialType::SSH_KEY) && ssh_key_idx < ssh_keys.len() {
            let (ref private, ref public) = ssh_keys[ssh_key_idx];
            ssh_key_idx += 1;
            let username = username.unwrap();
            return git2::Cred::ssh_key(username, public.as_deref(), private, None);
        }

        // 3. Try git credential helper (for HTTPS)
        if allowed.contains(git2::CredentialType::USER_PASS_PLAINTEXT) && !tried_cred_helper {
            tried_cred_helper = true;
            return git2::Cred::credential_helper(&config, url, username);
        }

        // 4. Try default credentials
        if allowed.contains(git2::CredentialType::DEFAULT) && !tried_default {
            tried_default = true;
            return git2::Cred::default();
        }

        Err(git2::Error::from_str("No authentication method succeeded"))
    })
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use crate::git::{
        find_last_commit, is_passphrase_protected, ssh_key_paths_in, Git, GitManagement,
        OPENSSH_MAGIC,
    };
    use git2::{BranchType, Repository, RepositoryInitOptions, Status};
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_git__init__valid_repo() {
        let mut git = Git::default();
        // Valid repo
        let (dir, _repo, _file) = repo_init();

        let actual = git.init(dir.path().to_str().unwrap());

        assert!(actual.is_ok());
    }

    #[test]
    fn test_git__init__invalid_repo() {
        let mut git = Git::default();
        // Invalid repo
        let dir = TempDir::new().unwrap();

        let actual = git.init(dir.path().to_str().unwrap());

        assert!(actual.is_err());
    }

    #[test]
    fn test_git__checkout_branch__missing_branch() {
        let mut git = Git::default();
        let (dir, repo, _file) = repo_init();
        git.init(dir.path().to_str().unwrap()).unwrap();

        // This will create a new branch
        git.checkout_branch("new-branch-name").unwrap();

        let actual = repo.find_branch("new-branch-name", BranchType::Local);

        assert!(actual.is_ok());
    }

    #[test]
    fn test_git__checkout_branch__success() {
        let mut git = Git::default();
        let (dir, repo, _file) = repo_init();
        git.init(dir.path().to_str().unwrap()).unwrap();

        let before = repo.head();
        assert_eq!(before.unwrap().name().unwrap(), "refs/heads/main");

        git.checkout_branch("new-branch-name").unwrap();

        let after = repo.head();

        assert!(after.is_ok());
        assert_eq!(after.unwrap().name().unwrap(), "refs/heads/new-branch-name");
    }

    #[test]
    fn test_git__ssh_key_paths_in__prefers_ed25519_and_pairs_public_key() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("id_rsa"), "unencrypted").unwrap();
        std::fs::write(dir.path().join("id_rsa.pub"), "pub").unwrap();
        std::fs::write(dir.path().join("id_ed25519"), "unencrypted").unwrap();

        let actual = ssh_key_paths_in(dir.path());

        // ed25519 is preferred, and its missing .pub is reported as None
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0].0, dir.path().join("id_ed25519"));
        assert_eq!(actual[0].1, None);
        assert_eq!(actual[1].0, dir.path().join("id_rsa"));
        assert_eq!(actual[1].1, Some(dir.path().join("id_rsa.pub")));
    }

    #[test]
    fn test_git__ssh_key_paths_in__no_keys() {
        let dir = TempDir::new().unwrap();

        assert!(ssh_key_paths_in(dir.path()).is_empty());
    }

    /// Build PEM armour around `body` at runtime.
    ///
    /// Assembled rather than written literally so these fixtures don't trip
    /// secret scanners -- the bodies below carry no key material.
    fn armour(label: &str, body: &str) -> String {
        let dashes = "-".repeat(5);
        format!("{dashes}BEGIN {label}{dashes}\n{body}\n{dashes}END {label}{dashes}")
    }

    #[test]
    fn test_git__ssh_key_paths_in__skips_passphrase_protected_key() {
        let dir = TempDir::new().unwrap();
        // libgit2 aborts the whole credential chain on an unloadable key, so
        // an encrypted one must never be offered
        std::fs::write(
            dir.path().join("id_ed25519"),
            armour("RSA PRIVATE KEY", "Proc-Type: 4,ENCRYPTED"),
        )
        .unwrap();
        std::fs::write(dir.path().join("id_rsa"), "unencrypted").unwrap();

        let actual = ssh_key_paths_in(dir.path());

        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0].0, dir.path().join("id_rsa"));
    }

    #[test]
    fn test_git__is_passphrase_protected__openssh_binary() {
        let unencrypted = [OPENSSH_MAGIC, &[0, 0, 0, 4], b"none"].concat();
        assert!(!is_passphrase_protected(&unencrypted));

        let encrypted = [OPENSSH_MAGIC, &[0, 0, 0, 10], b"aes256-ctr"].concat();
        assert!(is_passphrase_protected(&encrypted));

        // Truncated header is unusable, so treat it as protected
        let truncated = [OPENSSH_MAGIC, &[0]].concat();
        assert!(is_passphrase_protected(&truncated));
    }

    #[test]
    fn test_git__is_passphrase_protected__armoured_and_pem() {
        // Base64 of the openssh-key-v1 header with cipher "none"
        let unencrypted = armour("OPENSSH PRIVATE KEY", "b3BlbnNzaC1rZXktdjEAAAAABG5vbmU=");
        assert!(!is_passphrase_protected(unencrypted.as_bytes()));

        // Same header, but cipher "aes256-ctr"
        let encrypted = armour(
            "OPENSSH PRIVATE KEY",
            "b3BlbnNzaC1rZXktdjEAAAAACmFlczI1Ni1jdHIAAAAGYmNyeXB0",
        );
        assert!(is_passphrase_protected(encrypted.as_bytes()));

        let pkcs8 = armour("ENCRYPTED PRIVATE KEY", "");
        assert!(is_passphrase_protected(pkcs8.as_bytes()));

        let plain_pem = armour("RSA PRIVATE KEY", "MIIEow==");
        assert!(!is_passphrase_protected(plain_pem.as_bytes()));
    }

    #[test]
    fn test_git__add__success() {
        let mut git = Git::default();
        let (dir, repo, _file) = repo_init();
        git.init(dir.path().to_str().unwrap()).unwrap();

        let statuses_before = repo.statuses(None).unwrap();
        let before = statuses_before.get(0).unwrap();
        assert_eq!(before.status(), Status::WT_NEW);

        git.add("README.md").unwrap();

        let statuses_after = repo.statuses(None).unwrap();
        let after = statuses_after.get(0).unwrap();
        assert_eq!(after.status(), Status::INDEX_NEW);
    }

    #[test]
    fn test_git__commit__success() {
        let mut git = Git::default();
        let (dir, _repo, _file) = repo_init();
        git.init(dir.path().to_str().unwrap()).unwrap();

        // Initial commit
        let before = find_last_commit(git.repo.as_ref().unwrap());
        assert_eq!(before.unwrap().summary().unwrap(), "initial-msg");

        git.add("README.md").unwrap();
        git.commit("some-subject").unwrap();

        let after = find_last_commit(git.repo.as_ref().unwrap());
        assert_eq!(after.unwrap().summary().unwrap(), "some-subject");
    }

    fn repo_init() -> (TempDir, Repository, NamedTempFile) {
        let td = TempDir::new().unwrap();
        let mut opts = RepositoryInitOptions::new();
        opts.initial_head("main");
        let repo = Repository::init_opts(td.path(), &opts).unwrap();

        // Create README.md file
        let file = tempfile::Builder::new()
            .prefix("README")
            .suffix(".md")
            .rand_bytes(0)
            .tempfile_in(td.path())
            .unwrap();
        {
            // Set basic config
            let mut config = repo.config().unwrap();
            config.set_str("user.name", "some-name").unwrap();
            config.set_str("user.email", "some-email").unwrap();

            // Make initial commit
            let mut index = repo.index().unwrap();
            let id = index.write_tree().unwrap();
            let tree = repo.find_tree(id).unwrap();
            let sig = repo.signature().unwrap();
            repo.commit(Some("HEAD"), &sig, &sig, "initial-msg", &tree, &[])
                .unwrap();
        }
        // Return file to not drop it and make it disappear
        (td, repo, file)
    }
}
