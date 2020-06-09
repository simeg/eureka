pub enum CliFlag {
    ClearRepo,
    View,
    ShortView,
}

pub enum ConfigFile {
    Repo,
}

impl CliFlag {
    pub fn value(&self) -> &str {
        match *self {
            CliFlag::ClearRepo => "clear-repo",
            CliFlag::View => "view",
            CliFlag::ShortView => "v",
        }
    }
}

impl ConfigFile {
    pub fn value(&self) -> &str {
        match *self {
            // These represents files so underscore is preferred
            ConfigFile::Repo => "repo_path",
        }
    }
}
