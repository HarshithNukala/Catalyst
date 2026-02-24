use gpui:: Entity;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct PluginContext {
    pub config: Value,
    // pub app: Entity<crate::app::LauncherApp>
}

impl PluginContext {
    // pub fn new(config: Value, app: Entity<crate::app::LauncherApp>) -> Self {
    //     Self {
    //         config,
    //         app
    //     }
    // }
    // pub fn get_config<T>(&self, key: &str) -> Option<T>
    // where
    //     T: serde::de::DeserializeOwned,
    // {
    //     self.config
    //         .get(key)
    //         .and_then(|v| serde_json::from_value(v.clone()).ok())
    // }
}