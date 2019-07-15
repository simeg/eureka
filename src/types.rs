pub enum CliFlag {
    ClearRepo,
    ClearEditor,
    View,
    ShortView,
}

pub enum ConfigFile {
    Repo,
    Editor,
}

impl CliFlag {
    pub fn value(&self) -> &str {
        match *self {
            CliFlag::ClearRepo => "clear-repo",
            CliFlag::ClearEditor => "clear-editor",
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
            ConfigFile::Editor => "editor_path",
        }
    }
}
