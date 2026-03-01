use std::sync::Arc;
use async_trait::async_trait;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::core::plugin::{Plugin, PluginContext, Trigger};
use crate::core::model::{Action, BuiltInIcon, ResultIcon, ResultItem};

pub struct TestingImplicitPlugin {}

impl TestingImplicitPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Plugin for TestingImplicitPlugin {
    fn id(&self) -> &str {
        "testing_implicit"
    }
    fn name(&self) -> &str {
        "Testing Implicit Plugin"
    }
    fn description(&self) -> &str {
        "A plugin for testing implicit triggers."
    }
    fn trigger(&self) -> Trigger {
        Trigger::Implicit
    }
    async fn search(&self, query: &str, _context: &PluginContext) -> Vec<ResultItem> {
        let matcher = SkimMatcherV2::default();
        let score = matcher.fuzzy_match(&self.name().to_lowercase(), query);
        if let Some(_) = score {
            let mut results = Vec::new();
            results.push(ResultItem::new(
                "test_implicit",
                "Test Implicit",
                Action::None,
                self.id().to_string()
            )
            .with_subtitle("Test Implicit")
            .with_icon(ResultIcon::BuiltIn(BuiltInIcon::Search))
            .with_score(-10.0));
            return results
        }
        Vec::new()
    }
    async fn execute(&self, action: &Action, _context: &PluginContext) -> anyhow::Result<()> {
        if let Action::None = action {
            println!("Test Implicit executed");
        }
        Ok(())
    }
}