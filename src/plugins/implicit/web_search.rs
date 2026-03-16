use std::result;
use std::sync::Arc;
use async_trait::async_trait;
use google_search::search;

use crate::core::plugin::{Plugin, PluginContext, Trigger};
use crate::core::model::{Action, BuiltInIcon, ResultIcon, ResultItem};

pub struct WebSearchPlugin {}

impl WebSearchPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Plugin for WebSearchPlugin {
    fn id(&self) -> &str {
        "web_search"
    }
    fn name(&self) -> &str {
        "Web Search"
    }
    fn description(&self) -> &str {
        "A plugin for searching the web."
    }
    fn trigger(&self) -> Trigger {
        Trigger::Implicit
    }
    async fn search(&self, query: &str, _context: &PluginContext) -> Vec<ResultItem> {
        let mut results = Vec::new();
        results.push(
            ResultItem::new(
                "web search",
                query.to_string(),
                Action::OpenUrl(query.to_string()),
                self.id().to_string()
            )
            .with_subtitle("Search the web")
            .with_icon(ResultIcon::BuiltIn(BuiltInIcon::Search))
            .with_score(-100.0)
        );
        results
    }
    async fn execute(&self, action: &Action, _context: &PluginContext) -> anyhow::Result<()> {
        if let Action::OpenUrl(url) = action {
            let _ = search(url);
            println!("Search results for {}", url);
        }
        Ok(())
    }
}