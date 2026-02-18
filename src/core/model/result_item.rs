use gpui::SharedString;
use serde::{Deserialize, Serialize};

use super::Action;

#[derive(Debug, Clone, PartialEq)]
pub enum ResultIcon {
    Path(String),
    Emoji(String),
    AppIcon(String),
    BuiltIn(BuiltInIcon),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInIcon {
    Calculator,
    Search,
    File,
    Folder,
    Terminal,
    Settings,
    AI,
    Web,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResultItem {
    pub id: String,
    pub title: SharedString,
    pub subtitle: Option<SharedString>,
    pub icon: ResultIcon,
    pub action: Action,
    pub plugin_id: SharedString,
    pub score: f32,
    pub metadata: Option<serde_json::Value>,
}

impl ResultItem {
    pub fn new(
            id: impl Into<String>,
            title: impl Into<SharedString>, 
            action: Action, 
            plugin_id: impl Into<SharedString>) -> Self {
                Self {
                    id: id.into(),
                    title: title.into(),
                    subtitle: None,
                    icon: ResultIcon::BuiltIn(BuiltInIcon::Search),
                    action: action,
                    plugin_id: plugin_id.into(),
                    score: 0.0,
                    metadata: None,
                }
    }
    pub fn with_subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }
    pub fn with_score(mut self, score: f32) -> Self {
        self.score = score;
        self
    }
    pub fn with_metadata(mut self, metadata: serde_json:: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn with_icon(mut self, icon: ResultIcon) -> Self {
        self.icon = icon;
        self
    }
}