use std::sync::Arc;
use async_trait::async_trait;
use crate::core::plugin::{Plugin, PluginContext, Trigger};
use crate::core::model::{Action, BuiltInIcon, ResultIcon, ResultItem};
use crate::platform::windows::app_index::AppIndex;

pub struct AppSearchPlugin {
    index: Arc<AppIndex>
}

impl AppSearchPlugin {
    pub fn new() -> Self {
        Self {
            index: Arc::new(AppIndex::build()),
        }
    }
}

#[async_trait]
impl Plugin for AppSearchPlugin {
    fn id(&self) -> &str {
        "app_search"
    }
    fn name(&self) -> &str {
        "App Search"
    }
    fn description(&self) -> &str {
        "A plugin for launching applications."
    }
    fn trigger(&self) -> Trigger {
        Trigger::Implicit
    }
    async fn search(&self, query: &str, _context: &PluginContext) -> Vec<ResultItem> {
        let matches = self.index.search(query);
        matches.into_iter().take(10).map(|app| {
            let icon = match &app.icon {
                Some(path) => ResultIcon::Path(path.to_string_lossy().to_string()),
                None => ResultIcon::BuiltIn(BuiltInIcon::App)
            };
            ResultItem::new(
                app.path.to_string_lossy().to_string(),
                app.name.clone(),
                Action::LaunchApp {
                    path: app.path.clone(),
                    args: vec![],
                },
                self.id().to_string()
            )
            .with_subtitle("Application")
            .with_icon(icon)
        }).collect()
    }
    async fn execute(&self, action: &Action, _context: &PluginContext) -> anyhow::Result<()> {
        if let Action::LaunchApp {path, args} = action {
            std::process::Command::new(path)
                .args(args)
                .spawn()?;
        }
        Ok(())
    }
}