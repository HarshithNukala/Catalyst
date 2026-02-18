use gpui::Global;
use std::sync::Arc;

use crate::core::{
    config::Config,
    engine::{ActionDispatcher, QueryEngine},
    plugin::PluginRegistry,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub dispatcher: Arc<ActionDispatcher>,
    pub query_engine: Arc<QueryEngine>,
    pub registry: Arc<PluginRegistry>
}

impl Global for AppState {}

impl AppState {
    pub async fn new(config: Config, registry: Arc<PluginRegistry>) -> Self {
        let config = Arc::new(config);
        let query_engine = Arc::new(QueryEngine::new(registry.clone()).await);
        let dispatcher = Arc::new(ActionDispatcher::new(registry.clone()));
        Self {
            config,
            dispatcher,
            query_engine,
            registry
        }
    }
}

