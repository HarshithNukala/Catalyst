use std::result;
use std::sync::Arc;
use async_trait::async_trait;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use local_ip_address::local_ip;
use public_ip::addr;
use arboard::Clipboard;

use crate::core::plugin::{Plugin, PluginContext, Trigger};
use crate::core::model::{Action, BuiltInIcon, ResultIcon, ResultItem};


pub struct TestingExplicitPlugin {}

impl TestingExplicitPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Plugin for TestingExplicitPlugin {
    fn id(&self) -> &str {
        "testing_explicit"
    }
    fn name(&self) -> &str {
        "Testing Explicit Plugin"
    }
    fn description(&self) -> &str {
        "A plugin for testing explicit triggers."
    }
    fn trigger(&self) -> Trigger {
        Trigger::Explicit { keyword: "test".to_string() }
    }
    async fn search(&self, query: &str, _context: &PluginContext) -> Vec<ResultItem> {
        let private_ip = local_ip().unwrap();
        let public_ip = addr().await.unwrap();
        let mut results = Vec::new();
        results.push(
            ResultItem::new(
                "test explicit",
                public_ip.to_string(),
                Action::CopyToClipboard(public_ip.to_string()),
                self.id().to_string()
            )
            .with_subtitle("Public IP")
            .with_icon(ResultIcon::BuiltIn(BuiltInIcon::Search))
            .with_score(100.0)
        );
        results.push(
            ResultItem::new(
                "test explicit",
                private_ip.to_string(),
                Action::CopyToClipboard(private_ip.to_string()),
                self.id().to_string()
            )
            .with_subtitle("Private IP")
            .with_icon(ResultIcon::BuiltIn(BuiltInIcon::Search))
            .with_score(100.0)
        );
        results
    }
    async fn execute(&self, action: &Action, _context: &PluginContext) -> anyhow::Result<()> {
        let mut clipboard = Clipboard::new()?;
        if let Action::CopyToClipboard(ip) = action {
            let _ = clipboard.set_text(ip)?;
            println!("Ip address copied to clipboard: {}", ip);
        }
        Ok(())
    }
}