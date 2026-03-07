use async_trait::async_trait;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::core::plugin::{Plugin, PluginContext, Trigger};
use crate::core::model::{Action, BuiltInIcon, ResultIcon, ResultItem};

pub struct ExitPlugin {}

impl ExitPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Plugin for ExitPlugin {
    fn id(&self) -> &str {
        "exit"
    }
    fn name(&self) -> &str {
        "Exit Plugin"
    }
    fn description(&self) -> &str {
        "A plugin for exiting the application."
    }
    fn trigger(&self) -> Trigger {
        Trigger::Implicit
    }
    async fn search(&self, query: &str, _context: &PluginContext) -> Vec<ResultItem> {
        let mut results = Vec::new();
        let matcher = SkimMatcherV2::default();
        
        if let Some(_) = matcher.fuzzy_match("exit", query) {
            results.push(
            ResultItem::new(
                "Exit",
                "Exit",
                Action::Exit,
                self.id().to_string()
            )
            .with_subtitle("Exit the application")
            .with_icon(ResultIcon::BuiltIn(BuiltInIcon::Exit))
            .with_score(100.0)
            );
            return results;
        }
        Vec::new()
    }
    async fn execute(&self, action: &Action, _context: &PluginContext) -> anyhow::Result<()> {
        std::process::exit(0);
    }
}
