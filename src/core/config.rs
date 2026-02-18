use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub appearance: AppearanceConfig,
    pub hotkeys: HotkeysConfig,
    pub plugins: PluginsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub max_results: usize,
    telemetry: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub theme: String,
    pub window_width: u32,
    pub window_height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeysConfig {
    pub show_launcher: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginsConfig {
    pub calculator: Option<serde_json::Value>,
    pub ai: Option<serde_json::Value>,
    pub custom: std::collections::HashMap<String, serde_json::Value>,
}

impl Config {
    pub fn laod() -> anyhow::Result<Self> {
        let config_path = PathBuf::from("config/default.toml");
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            log::warn!("Config file not found, using defaults.");
            Ok(Self::default())
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                max_results: 10,
                telemetry: false,
            },
            appearance: AppearanceConfig {
                theme: "light".to_string(),
                window_width: 600,
                window_height: 500,
            },
            hotkeys: HotkeysConfig {
                show_launcher: "Alt+Space".to_string(),
            },
            plugins: PluginsConfig {
                calculator: None,
                ai: None,
                custom: std::collections::HashMap::new(),
            },
        }
    }
}