use async_trait::async_trait;

use crate::core::model::{Action, ResultItem};
use super::{PluginContext, Trigger};

#[async_trait]
pub trait Plugin: Send + Sync + 'static {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn trigger(&self) -> Trigger;
    async fn search(&self, query: &str, context: &PluginContext) -> Vec<ResultItem>;
    async fn execute(&self, action: &Action, context: &PluginContext) -> anyhow::Result<()>;
    async fn initialize(&mut self, _context: &PluginContext) -> anyhow::Result<()> {Ok(())}
    async fn cleanup(&mut self) -> anyhow::Result<()> {Ok(())}
    fn requires_initialization(&self) -> bool {false}
    fn priority(&self) -> i32 {0}
}