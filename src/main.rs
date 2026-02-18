use std::process::Child;

use gpui:: {
    App, Application, Bounds, Context, Entity, FocusHandle, Focusable, KeyBinding, KeyDownEvent, Point, SharedString, Size, Subscription, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, rgb, size
};

use gpui_component:: {
    highlighter::Language, input::{Input, InputEvent, InputState}, *
};
use gpui_component_assets::Assets;

actions!(Input_element, [HideApp]);

struct Input_element {
    input_state: Entity<InputState>,
    text: SharedString,
    _subscription: Vec<Subscription>,
    focus_handle: FocusHandle,
}

impl Focusable for Input_element {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Input_element {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Type here to search")
        });
        input_state.update(cx, |state, cx| {
            state.focus(window, cx);
        });
        let is_focused = input_state.focus_handle(cx).is_focused(window);

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
                    let value = input_state.read(cx).value();
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
        self.input_state.update(cx, |state, cx| {
            state.set_value(String::new(), window, cx);
        });
        cx.hide();
        cx.stop_propagation();
    }
}

impl Render for Input_element {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let input_state = &self.input_state;
        let is_focused = input_state.focus_handle(cx).is_focused(window);
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x505050))
            .p_0()
            .m_0()
            .gap_0()
            .justify_center()
            .items_center()
            .key_context("Input_element")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::hide_app))
            // .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
            //     let key = event.keystroke.key.as_str();
            //     match key {
            //         "escape" => {
            //             cx.hide();
            //             println!("escaped!!!");
            //         }
            //         _ => {}
            //     }
            // }))
            .child(
                Input::new(&self.input_state)
                .large()
                // .h(px(50.0))
                .w(px(600.0))
                .line_height(px(40.0))
                .text_size(px(30.0))
                .m_1()
                .bg(rgb(0x505050))
                .border_0()
            )
            // .child(format!("Hello, {}", self.text))
    }
}



fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        gpui_component::init(cx);

        let window_width = px(610.0);
        let window_height = px(50.0);
        let displays = cx.displays();
        let primary_display = &displays[0];
        let screen_bounds = primary_display.bounds();
        let x = screen_bounds.center().x - window_width / 2.0;
        let y = screen_bounds.center().y - (screen_bounds.size.height * 0.2) - window_height / 2.0;
        
        // let bounds = Bounds::centered(None, size(window_width, window_height), cx);
        let bounds = Bounds {
            origin: Point::new(x, y),
            size: Size {
                width: window_width,
                height: window_height
            }
        };
        cx.open_window(
            WindowOptions {
                titlebar: None,
                focus: true,
                show: true,
                is_movable: false,
                is_resizable: false,
                // kind: gpui::WindowKind::PopUp,
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let input = cx.new(|cx| Input_element::new(window, cx));
                cx.new(|cx| Root::new(input, window, cx))
            },
        )
        .unwrap();
        cx.activate(true);
    })
}
