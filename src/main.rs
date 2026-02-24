#![allow(unused_imports)]

mod core;
mod platform;
mod plugins;
mod services;
mod ui;

use std::process::Child;

use gpui:: {
    App, Application, AsyncApp, Bounds, Context, Entity, FocusHandle, Focusable, KeyBinding, KeyDownEvent, Point, SharedString, Size, Subscription, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, rgb, size, AssetSource
};

use adabraka_ui::prelude::*;
use std::path::PathBuf;
use adabraka_ui::components::input::{Input, InputEvent};
use adabraka_ui::components::input_state::InputState;
use futures::channel::mpsc;
use futures::StreamExt;




struct Assets {
    base: PathBuf,
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries.filter_map(|entry| {
                    entry.ok().map(|e| {
                        SharedString::from(e.file_name().to_string_lossy().to_string())
                    })
                }).collect()
            })
            .map_err(|err| err.into())
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        adabraka_ui::init(cx);
        adabraka_ui::set_icon_base_path("assets/icons");
        install_theme(cx, Theme::dark());

        let registry = std::sync::Arc::new(crate::core::plugin::PluginRegistry::new());
        futures::executor::block_on(async {
            let plugin: std::sync::Arc<dyn crate::core::plugin::Plugin> = std::sync::Arc::new(crate::plugins::implicit::app_search::AppSearchPlugin::new());
            let _ = registry.register(plugin).await;
        });
        let engine = std::sync::Arc::new(crate::core::engine::QueryEngine::new(registry.clone()));
        
        let window_width = px(610.0);
        let window_height = px(500.0);
        let displays = cx.displays();
        let primary_display = &displays[0];
        let screen_bounds = primary_display.bounds();
        let x = screen_bounds.center().x - window_width / 2.0;
        let y = screen_bounds.center().y - (screen_bounds.size.height * 0.2) - window_height / 2.0;
        
        let bounds = Bounds {
            origin: Point::new(x, y),
            size: Size {
                width: window_width,
                height: window_height
            }
        };
        
        let window_handle = cx.open_window(
            WindowOptions {
                titlebar: None,
                focus: true,
                show: true,
                is_movable: false,
                is_resizable: false,
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| ui::components::search_bar::Input_element::new(window, cx, engine.clone()))
            },
        )
        .unwrap();

        let mut rx = platform::windows::hotkey::register_hotkey();
        let async_cx = cx.to_async();
        cx.foreground_executor().spawn(async move {
            while let Some(()) = rx.next().await {
                let _ = async_cx.update(|cx| {
                    let _ = window_handle.update(cx, |view, window, _cx| {
                        println!("Showing app");
                        _cx.activate(true);
                        window.show_window();
                        view.input_state.focus_handle(_cx).focus(window);
                    });
                });
            }
        }).detach();

        cx.activate(true);
    })
}