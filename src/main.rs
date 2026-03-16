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
// use windows::Win32::UI::WindowsAndMessaging::HWND_TOPMOST;
use std::path::PathBuf;
use adabraka_ui::components::input::{Input, InputEvent};
use adabraka_ui::components::input_state::InputState;
use futures::channel::mpsc;
use futures::StreamExt;
use gpui::WindowKind;
use gpui::WindowBackgroundAppearance::Transparent;

use raw_window_handle::HasWindowHandle;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW,
};


use crate::core::engine::ActionDispatcher;

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
        let dispatcher = std::sync::Arc::new(ActionDispatcher::new(registry.clone()));
        futures::executor::block_on(async {
            let app_search_plugin: std::sync::Arc<dyn crate::core::plugin::Plugin> = std::sync::Arc::new(crate::plugins::implicit::app_search::AppSearchPlugin::new());
            let _ = registry.register(app_search_plugin).await;
            let testing_implicit_plugin: std::sync::Arc<dyn crate::core::plugin::Plugin> = std::sync::Arc::new(crate::plugins::implicit::testing_implicit::TestingImplicitPlugin::new());
            let _ = registry.register(testing_implicit_plugin).await;
            let testing_explicit_plugin: std::sync::Arc<dyn crate::core::plugin::Plugin> = std::sync::Arc::new(crate::plugins::explicit::testing_explicit::TestingExplicitPlugin::new());
            let _ = registry.register(testing_explicit_plugin).await;
            let ip_plugin: std::sync::Arc<dyn crate::core::plugin::Plugin> = std::sync::Arc::new(crate::plugins::explicit::ip::IpPlugin::new());
            let _ = registry.register(ip_plugin).await;
            let calculator_plugin: std::sync::Arc<dyn crate::core::plugin::Plugin> = std::sync::Arc::new(crate::plugins::implicit::calculator::CalculatorPlugin::new());
            let _ = registry.register(calculator_plugin).await;
            let web_search_plugin: std::sync::Arc<dyn crate::core::plugin::Plugin> = std::sync::Arc::new(crate::plugins::implicit::web_search::WebSearchPlugin::new());
            let _ = registry.register(web_search_plugin).await;
            let dictionary_plugin: std::sync::Arc<dyn crate::core::plugin::Plugin> = std::sync::Arc::new(crate::plugins::explicit::dictionary::DictionaryPlugin::new());
            let _ = registry.register(dictionary_plugin).await;
            let system_commands_plugin: std::sync::Arc<dyn crate::core::plugin::Plugin> = std::sync::Arc::new(crate::plugins::implicit::system_commands::SystemCommandsPlugin::new());
            let _ = registry.register(system_commands_plugin).await;
            let clipboard_plugin: std::sync::Arc<dyn crate::core::plugin::Plugin> = std::sync::Arc::new(crate::plugins::explicit::clipboard::ClipboardPlugin::new());
            let _ = registry.register(clipboard_plugin).await;
            let exit_plugin: std::sync::Arc<dyn crate::core::plugin::Plugin> = std::sync::Arc::new(crate::plugins::implicit::exit::ExitPlugin::new());
            let _ = registry.register(exit_plugin).await;
            let terminal_plugin: std::sync::Arc<dyn crate::core::plugin::Plugin> = std::sync::Arc::new(crate::plugins::explicit::terminal::TerminalPlugin::new());
            let _ = registry.register(terminal_plugin).await;
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
                // titlebar: Some(TitlebarOptions {
                //     title: Some("Catalyst".into()),
                //     ..Default::default()
                // }),
                titlebar: None,
                // kind: WindowKind::PopUp,
                window_background: gpui::WindowBackgroundAppearance::Transparent,
                focus: true,
                show: true,
                is_movable: false,
                is_resizable: false,
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| ui::components::search_bar::Input_element::new(window, cx, engine.clone(), dispatcher.clone()))
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
                        // if let Ok(handle) = window.window_handle() {
                        //     if let raw_window_handle::RawWindowHandle::Win32(win32_handle) = handle.as_raw() {
                        //         let hwnd = HWND(win32_handle.hwnd.get() as *mut _);
                        //         unsafe {
                        //             let _ = SetWindowPos(
                        //                 hwnd,
                        //                 Some(HWND_TOPMOST),
                        //                 0, 0, 0, 0,
                        //                 SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
                        //             );  
                        //         }
                        //     }
                        // }
                        view.input_state.focus_handle(_cx).focus(window);
                        window.dispatch_action(Box::new(adabraka_ui::components::input::SelectAll), _cx);
                        
                    });
                });
            }
        }).detach();

        cx.activate(true);
    })
}