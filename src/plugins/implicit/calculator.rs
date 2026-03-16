use std::result;
use std::sync::Arc;
use async_trait::async_trait;
use arboard::Clipboard;
use meval::eval_str;
use thousands::Separable;

use crate::core::plugin::{Plugin, PluginContext, Trigger};
use crate::core::model::{Action, BuiltInIcon, ResultIcon, ResultItem};

pub struct CalculatorPlugin {}

impl CalculatorPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Plugin for CalculatorPlugin {
    fn id(&self) -> &str {
        "calculator"
    }
    fn name(&self) -> &str {
        "Calculator"
    }
    fn description(&self) -> &str {
        "A plugin for performing calculations."
    }
    fn trigger(&self) -> Trigger {
        Trigger::Implicit
    }
    async fn search(&self, query: &str, _context: &PluginContext) -> Vec<ResultItem> {
        let mut results = Vec::new();
        let output = eval_str(query);
        if let Ok(output) = output {
            let formatted_output = output.separate_with_commas();
            results.push(ResultItem::new(
                "calculator",
                formatted_output,
                Action::CopyToClipboard(format!("{}", output)),
                self.id().to_string()
            )
            .with_subtitle("Calculator")
            .with_icon(ResultIcon::BuiltIn(BuiltInIcon::Calculator))
            .with_score(100.0)
        );
        }
        results
    }
    async fn execute(&self, action: &Action, _context: &PluginContext) -> anyhow::Result<()> {
        let mut clipboard = Clipboard::new().unwrap();
        if let Action::CopyToClipboard(output) = action {
            clipboard.set_text(output).unwrap();
            println!("Result copied to clipboard");
        }
        Ok(())
    }
}