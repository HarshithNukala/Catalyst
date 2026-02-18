use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{Plugin, Trigger};

pub struct PluginRegistry {
    plugins: Arc<RwLock<HashMap<String, Arc<dyn Plugin>>>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub async fn register(&self, plugin: Arc<dyn Plugin>) -> anyhow::Result<()> {
        let mut plugins = self.plugins.write().await;
        let id = plugin.id().to_string();
        if plugins.contains_key(&id) {
            anyhow::bail!("Plugin with ID {} already registered.", id);
        }
        log::info!("Registering plugin: {} ({})", plugin.name(), id);
        plugins.insert(id, plugin);
        Ok(())
    }
    pub async fn unregister(&self, plugin_id: &str) -> anyhow::Result<()> {
        let mut plugins = self.plugins.write().await;
        plugins.remove(plugin_id).ok_or_else(|| anyhow::anyhow!("Plugin {} not found.", plugin_id))?;
        Ok(())
    }
    pub async fn get(&self, plugin_id: &str) -> Option<Arc<dyn Plugin>> {
        let plugins = self.plugins.read().await;
        plugins.get(plugin_id).cloned()
    }
    pub async fn match_plugins(&self, query: &str) -> Vec<Arc<dyn Plugin>> {
        let plugins = self.plugins.read().await;
        let mut matched = Vec::new();
        for plugin in plugins.values() {
            if plugin.trigger().matches(query).is_some() {
                matched.push(plugin.clone());
            }
        }
        matched.sort_by(|a, b| b.priority().cmp(&a.priority()));
        matched
    }
    pub async fn implicit_plugins(&self) -> Vec<Arc<dyn Plugin>> {
        let plugins = self.plugins.read().await;
        plugins.values().filter(|p| p.trigger().is_implicit()).cloned().collect()
    }
    pub async fn all_plugins(&self) -> Vec<Arc<dyn Plugin>> {
        let plugins = self.plugins.read().await;
        plugins.values().cloned().collect()
    }
    pub async fn count(&self) -> usize {
        let plugins = self.plugins.read().await;
        plugins.len()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}