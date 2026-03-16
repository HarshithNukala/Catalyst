use gpui:: {
    App, Application, AssetSource, AsyncApp, Bounds, Context, Entity, FocusHandle, Focusable, KeyBinding, KeyDownEvent, Point, ScrollHandle, SharedString, Size, Subscription, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, rgb, size
};

use adabraka_ui::components::input::{Input, InputEvent, InputVariant};
use adabraka_ui::components::input_state::InputState;

pub struct SearchBar {
    pub input_state: Entity<InputState>,
}

impl SearchBar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| InputState::new(cx));
        
        Self {
            input_state,
        }
    }
}
