use std::result;
use std::sync::{Arc, RwLock};
use anyhow::Ok;
use async_trait::async_trait;
use termlauncher::Application;
use std::os::windows::process::CommandExt;

use crate::core::plugin::{Plugin, PluginContext, Trigger};
use crate::core::model::{Action, BuiltInIcon, ResultIcon, ResultItem};

pub struct TerminalPlugin {}

impl TerminalPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Plugin for TerminalPlugin {
    fn id(&self) -> &str {
        "terminal"
    }
    fn name(&self) -> &str {
        "Terminal Plugin"
    }
    fn description(&self) -> &str {
        "A plugin for running terminal commands."
    }
    fn trigger(&self) -> Trigger {
        Trigger::Explicit { keyword: "/".to_string() }
    }
    async fn search(&self, query: &str, _context: &PluginContext) -> Vec<ResultItem> {
        let mut results = Vec::new();
        results.push(
            ResultItem::new(
                &format!("Run: {}", query),
                query.to_string(),
                Action::ExecuteCommand { command: query.to_string(), args: vec![] },
                self.id().to_string()
            )
            .with_icon(ResultIcon::BuiltIn(BuiltInIcon::Terminal))
            .with_score(100.0)
        );
        results
    }
    async fn execute(&self, action: &Action, _context: &PluginContext) -> anyhow::Result<()> {
        if let Action::ExecuteCommand { command, args } = action {
            let home = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\".to_string());
            let command = format!("{}; Write-Host 'Press any key to close this window'; $null = [Console]::ReadKey($true)", command);
            let _ = std::process::Command::new("pwsh")
                .args(["-Command", &command])
                .current_dir(home)
                .creation_flags(0x00000010) // Opens up a new terminal window
                .spawn()?;
            println!("Command executed: {}", command);
        }
        Ok(())
    }
}
