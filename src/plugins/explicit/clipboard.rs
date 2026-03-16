use async_trait::async_trait;
use arboard::Clipboard;

use crate::core::plugin::{Plugin, PluginContext, Trigger};
use crate::core::model::{Action, BuiltInIcon, ResultIcon, ResultItem};

pub struct ClipboardPlugin {}

impl ClipboardPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Plugin for ClipboardPlugin {
    fn id(&self) -> &str {
        "clipboard"
    }
    fn name(&self) -> &str {
        "Clipboard Plugin"
    }
    fn description(&self) -> &str {
        "A plugin for getting clipboard contents."
    }
    fn trigger(&self) -> Trigger {
        Trigger::Explicit { keyword: "clip".to_string() }
    }
    async fn search(&self, query: &str, _context: &PluginContext) -> Vec<ResultItem> {
        let mut results = Vec::new();
        results
    }
    async fn execute(&self, action: &Action, _context: &PluginContext) -> anyhow::Result<()> {
        let mut clipboard = Clipboard::new()?;
        if let Action::CopyToClipboard(text) = action {
            let _ = clipboard.set_text(text)?;
            println!("Text copied to clipboard: {}", text);
        }
        Ok(())
    }
}