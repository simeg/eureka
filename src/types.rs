pub enum CliFlag {
    ClearBranch,
    ClearRepo,
    ShortView,
    View,
}

#[derive(Eq, PartialEq, Debug)]
pub enum ConfigFile {
    Branch,
    Repo,
}

impl CliFlag {
    pub fn value(&self) -> &str {
        match *self {
            CliFlag::ClearBranch => "clear-branch",
            CliFlag::ClearRepo => "clear-repo",
            CliFlag::ShortView => "v",
            CliFlag::View => "view",
        }
    }
}

impl ConfigFile {
    pub fn value(&self) -> &str {
        match *self {
            // These represents files so underscore is preferred
            ConfigFile::Branch => "branch",
            ConfigFile::Repo => "repo_path",
        }
    }
}
