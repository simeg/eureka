pub enum CliFlag {
    ClearRepo,
    ClearEditor,
    View,
    ShortView,
}

pub enum ConfigType {
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

impl ConfigType {
    pub fn value(&self) -> &str {
        match *self {
            ConfigType::Repo => "repo_path",
            ConfigType::Editor => "editor_path",
        }
    }
}
