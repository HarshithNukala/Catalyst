use gpui::{AppContext, Task};
use tokio::task::JoinHandle;
use std::sync::Arc;

use crate::core::{
    plugin::{PluginRegistry, PluginContext},
    model::Action,
};

pub struct ActionDispatcher {
    registry: Arc<PluginRegistry>,
    runtime: Arc<tokio::runtime::Runtime>
}

impl ActionDispatcher {
    pub fn new(registry: Arc<PluginRegistry>) -> Self {
        Self {
            registry,
            runtime: Arc::new(tokio::runtime::Runtime::new().expect("Failed to create tokio runtime for ActionDispatcher.")),
        }
    }
    pub fn execute(
            &self, 
            plugin_id: String, 
            action: Action, 
            context: PluginContext, 
        ) -> tokio::task::JoinHandle<anyhow::Result<()>> {
            let registry = self.registry.clone();
            self.runtime.spawn(async move {
                log::info!("Executing action from plugin: {}", plugin_id);
                let plugin = registry.get(&plugin_id).await.ok_or_else(|| {
                    anyhow::anyhow!("Plugin {} not found.", plugin_id)
                })?;
                plugin.execute(&action, &context).await?;
                log::info!("Action executed successfully.");
                Ok(())
            })
    }
}