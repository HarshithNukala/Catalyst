use std::result;
use std::sync::Arc;
use async_trait::async_trait;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use system_shutdown;
use crate::core::plugin::{Plugin, PluginContext, Trigger};
use crate::core::model::{Action, BuiltInIcon, ResultIcon, ResultItem};

pub struct SystemCommandsPlugin {}

impl SystemCommandsPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Plugin for SystemCommandsPlugin {
    fn id(&self) -> &str {
        "system_commands"
    }
    fn name(&self) -> &str {
        "System Commands"
    }
    fn description(&self) -> &str {
        "A plugin for running system commands."
    }
    fn trigger(&self) -> Trigger {
        Trigger::Implicit
    }
    async fn search(&self, query: &str, _context: &PluginContext) -> Vec<ResultItem> {
        let mut results = Vec::new();
        let matcher = SkimMatcherV2::default();

        if let Some(_) = matcher.fuzzy_match("shutdown", query) {
            results.push(ResultItem::new(
            "system_commands",
            "Shutdown",
            Action::SystemCommand("shutdown".to_string()),
            self.id().to_string()
        )
        .with_subtitle("Shutdown the system")
        .with_icon(ResultIcon::BuiltIn(BuiltInIcon::Terminal))
        .with_score(25.0));
        return results
        }

        if let Some(_) = matcher.fuzzy_match("restart", query) {
            results.push(ResultItem::new(
            "system_commands",
            "Restart",
            Action::SystemCommand("restart".to_string()),
            self.id().to_string()
        )
        .with_subtitle("Restart the system")
        .with_icon(ResultIcon::BuiltIn(BuiltInIcon::Terminal))
        .with_score(25.0));
        return results
        }

        if let Some(_) = matcher.fuzzy_match("sleep", query) {
            results.push(ResultItem::new(
            "system_commands",
            "Sleep",
            Action::SystemCommand("sleep".to_string()),
            self.id().to_string()
        )
        .with_subtitle("Put the system to sleep")
        .with_icon(ResultIcon::BuiltIn(BuiltInIcon::Terminal))
        .with_score(25.0));
        return results
        }

        if let Some(_) = matcher.fuzzy_match("hibernate", query) {
            results.push(ResultItem::new(
            "system_commands",
            "Hibernate",
            Action::SystemCommand("hibernate".to_string()),
            self.id().to_string()
        )
        .with_subtitle("Put the system to hibernate")
        .with_icon(ResultIcon::BuiltIn(BuiltInIcon::Terminal))
        .with_score(25.0));
        return results
        }

        if let Some(_) = matcher.fuzzy_match("logout", query) {
            results.push(ResultItem::new(
            "system_commands",
            "Logout",
            Action::SystemCommand("logout".to_string()),
            self.id().to_string()
        )
        .with_subtitle("Log out of the system")
        .with_icon(ResultIcon::BuiltIn(BuiltInIcon::Terminal))
        .with_score(25.0));
        return results
        }

        Vec::new()
    }
    async fn execute(&self, action: &Action, _context: &PluginContext) -> anyhow::Result<()> {
        if let Action::SystemCommand(command) = action {
            match command.as_str() {
                "shutdown" => {
                    println!("Shutting down the system");
                    system_shutdown::shutdown();
                }
                "restart" => {
                    println!("Restarting the system");
                    system_shutdown::reboot();
                }
                "sleep" => {
                    println!("Putting the system to sleep");
                    system_shutdown::sleep();
                }
                "hibernate" => {
                    println!("Putting the system to hibernate");
                    system_shutdown::hibernate();
                }
                "logout" => {
                    println!("Logging out of the system");
                    system_shutdown::logout();
                }
                _ => {
                    println!("Unknown command: {}", command);
                }
            }
        }
        Ok(())
    }
}