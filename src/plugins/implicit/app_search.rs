use std::sync::Arc;
use async_trait::async_trait;
// use dirs::cache_dir;
use image::ImageReader;
use std::path::{Path, PathBuf};
use md5;

use crate::core::plugin::{Plugin, PluginContext, Trigger};
use crate::core::model::{Action, BuiltInIcon, ResultIcon, ResultItem};
use crate::platform::windows::app_index::AppIndex;

fn convert_ico_to_png(ico_path: &Path) -> Option<PathBuf> {
    let cache_dir = dirs::data_dir()?.join("Catalyst").join("icon_cache");
    std::fs::create_dir_all(&cache_dir).ok()?;
    
    let hash = format!("{:x}", md5::compute(ico_path.to_string_lossy().as_bytes()));
    let png_path = cache_dir.join(format!("{}.png", hash));
    
    if png_path.exists() {
        return Some(png_path);
    }
    // Skip .exe files - can't decode icons from them with the image crate
    let ext = ico_path.extension()?.to_string_lossy().to_lowercase();
    if ext == "exe" {
        return None;
    }
    
    let img = ImageReader::open(ico_path).ok()?.decode().ok()?;
    img.save_with_format(&png_path, image::ImageFormat::Png).ok()?;
    Some(png_path)
}


pub struct AppSearchPlugin {
    index: Arc<AppIndex>
}

impl AppSearchPlugin {
    pub fn new() -> Self {
        Self {
            index: Arc::new(AppIndex::build()),
        }
    }
}

#[async_trait]
impl Plugin for AppSearchPlugin {
    fn id(&self) -> &str {
        "app_search"
    }
    fn name(&self) -> &str {
        "App Search"
    }
    fn description(&self) -> &str {
        "A plugin for launching applications."
    }
    fn trigger(&self) -> Trigger {
        Trigger::Implicit
    }
    async fn search(&self, query: &str, _context: &PluginContext) -> Vec<ResultItem> {
        let matches = self.index.search(query);
        matches.into_iter().take(10).map(|app| {
            let icon = match &app.icon {
                Some(path) => {
                    if let Some(png_path) = convert_ico_to_png(path) {
                        ResultIcon::Path(png_path.to_string_lossy().to_string())
                    } else {
                        ResultIcon::BuiltIn(BuiltInIcon::App)
                    }
                }
                None => ResultIcon::BuiltIn(BuiltInIcon::App)
            };
            ResultItem::new(
                app.path.to_string_lossy().to_string(),
                app.name.clone(),
                Action::LaunchApp {
                    path: app.path.clone(),
                    args: vec![],
                },
                self.id().to_string()
            )
            .with_subtitle("Application")
            .with_icon(icon)
        }).collect()
    }
    async fn execute(&self, action: &Action, _context: &PluginContext) -> anyhow::Result<()> {
        if let Action::LaunchApp {path, args} = action {
            std::process::Command::new(path)
                .args(args)
                .spawn()?;
        }
        Ok(())
    }
}