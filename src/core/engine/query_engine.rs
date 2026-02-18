use std::sync::Arc;
use tokio::task::JoinSet;

use crate::core::{
    model::ResultItem,
    plugin::{PluginContext, PluginRegistry},
};

use super::Ranker;

pub struct QueryEngine {
    registry: Arc<PluginRegistry>,
    ranker: Ranker,
}

impl QueryEngine {
    pub async fn new(registry: Arc<PluginRegistry>) -> Self {
        Self {
            registry,
            ranker: Ranker::new()
        }
    }
    pub async fn search(&self, query: &str, context: &PluginContext) -> anyhow::Result<Vec<ResultItem>> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }
        let plugins = self.registry.match_plugins(query).await;
        if plugins.is_empty() {
            return Ok(Vec::new());
        }
        log::debug!("Searching {} for plugin for: {}", plugins.len(), query);
        let mut tasks = JoinSet::new();
        for plugin in plugins {
            let query = query.to_lowercase();
            let context_clone = context.clone();
            tasks.spawn(async move {
                let plugin_query = plugin.trigger().matches(&query).unwrap_or_default();
                let results = plugin.search(&plugin_query, &context_clone).await;
                (plugin.id().to_string(), results)
            });
        }
        let mut all_results = Vec::new();
        while let Some(result) = tasks.join_next().await {
            match result {
                Ok((plugin_id, mut results)) => {
                    log::debug!("Plugin '{}' returned {} results", plugin_id, results.len());
                    all_results.append(&mut results);
                }
                Err(e) => {
                    log::error!("Plugin search task failed: {}", e);
                }
            }
        }
        self.ranker.rank_results(&mut all_results, query);
        log::debug!("Total results: {}", all_results.len());
        Ok(all_results)
    }
}