pub enum CliFlag {
    ClearRepo,
    ClearEditor,
    View,
}

pub enum ConfigType {
    Repo,
    Editor,
}

impl CliFlag {
    pub fn value(&self) -> &str {
        match *self {
            CliFlag::ClearRepo => "clear_repo",
            CliFlag::ClearEditor => "clear_editor",
            CliFlag::View => "view",
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
