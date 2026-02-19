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

actions!(Input_element, [HideApp]);

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

struct Input_element {
    input_state: Entity<InputState>,
    text: SharedString,
    _subscription: Vec<Subscription>,
    focus_handle: FocusHandle,
}

impl Input_element {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| InputState::new(cx));
        input_state.focus_handle(cx).focus(window);
        
        let focus_handle = cx.focus_handle();
        
        cx.bind_keys([
            KeyBinding::new("escape", HideApp, None),
        ]);
        
        cx.observe_window_activation(window, |_this, _window, cx| {
            if !_window.is_window_active() {
                cx.hide();
            }
        }).detach();

        let _subscription = vec![cx.subscribe_in(&input_state, window, {
            let input_state = input_state.clone();
            move |this, _, ev: &InputEvent, _window, cx| match ev {
                InputEvent::Change => {
                    let value: String = input_state.read(cx).content().to_string();
                    this.text = value.into();
                    cx.notify();
                }
                _ => {}
            }
        })];
        
        Self {
            input_state,
            text: SharedString::default(),
            _subscription,
            focus_handle,
        }
    }
    
    fn hide_app(&mut self, _: &HideApp, window: &mut Window, cx: &mut Context<Self>) {
        println!("Hiding app!!");
        window.hide_window();
    }
}

impl Render for Input_element {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            // .bg(gpui::black())
            .p_0()
            .m_0()
            .gap_0()
            .justify_center()
            .items_center()
            .key_context("Input_element")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::hide_app))
            .child(
                Input::new(&self.input_state)
                    .placeholder("Type to search...")
                    .w(px(600.0))
                    .h(px(50.0))
                    .text_size(px(30.0))
                    .line_height(px(40.0))
                    .m_1()
                    .bg(rgb(0x505050))
                    .border_0()
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        adabraka_ui::init(cx);
        adabraka_ui::set_icon_base_path("assets/icons");
        install_theme(cx, Theme::dark());

        let window_width = px(610.0);
        let window_height = px(50.0);
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
                cx.new(|cx| Input_element::new(window, cx))
            },
        )
        .unwrap();

        // Set up global hotkey (Ctrl+Space) via Win32 API
        let (tx, mut rx) = mpsc::unbounded::<()>();

        std::thread::spawn(move || {
            unsafe {
                use windows::Win32::UI::Input::KeyboardAndMouse::{
                    RegisterHotKey, MOD_CONTROL, VK_SPACE,
                };
                use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY};

                let result = RegisterHotKey(None, 1, MOD_CONTROL, VK_SPACE.0 as u32);
                match result {
                    Ok(_) => println!("Global hotkey Ctrl+Space registered successfully"),
                    Err(e) => {
                        eprintln!("Failed to register global hotkey: {}", e);
                        return;
                    }
                }

                let mut msg = MSG::default();
                while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                    if msg.message == WM_HOTKEY {
                        let _ = tx.unbounded_send(());
                    }
                }
            }
        });

        // GPUI async task: listen for hotkey signal and show the window
        let async_cx = cx.to_async();
        cx.foreground_executor().spawn(async move {
            while let Some(()) = rx.next().await {
                let _ = async_cx.update(|cx| {
                    let _ = window_handle.update(cx, |view, window, _cx| {
                        _cx.activate(true);
                        window.show_window();
                        view.input_state.focus_handle(_cx).focus(window);
                        println!("Showing app via global hotkey!");
                    });
                });
            }
        })
        .detach();

        cx.activate(true);
    })
}