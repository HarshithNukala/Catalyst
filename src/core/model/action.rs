use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    LaunchApp {
        path: PathBuf,
        args: Vec<String>
    },
    OpenFile(PathBuf),
    OpenUrl(String),
    ExecuteCommand {
        command: String,
        args: Vec<String>
    },
    CopyToClipboard(String),
    InsertText(String),
    // For complex plugins like AI
    ShowPluginView {
        plugin_id: String
    },
    OpenSettings,
    // Custom action with json data
    Custom {
        action_type: String,
        data: serde_json::Value
    },
    // Do nothing
    None,
}

impl Action {
    pub fn is_immediate(&self) -> bool {
        !matches!(self, Action::ShowPluginView { .. })
    }
}