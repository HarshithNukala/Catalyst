use env_logger::fmt::style::Style;
use global_hotkey::hotkey::Modifiers;
use gpui:: {
    App, Application, AssetSource, AsyncApp, Bounds, Context, Entity, FocusHandle, Focusable, KeyBinding, KeyDownEvent, Point, ScrollHandle, SharedString, Size, Subscription, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, rgb, size
};
use std::result;
use std::sync::{Arc, Condvar};
use adabraka_ui::components::input::{Input, InputEvent, InputVariant};
use adabraka_ui::components::input_state::InputState;

use crate::core::engine::{ActionDispatcher, QueryEngine};
use crate::core::model::{Action, BuiltInIcon, ResultIcon, ResultItem};
use crate::core::plugin;
use crate::core::plugin::{PluginContext, PluginRegistry};
use crate::ui::components::result_list::ResultList;
use crate::ui::components::search_bar::SearchBar;

actions!(Input_element, [HideApp, ExecuteSelected, NavigateDown, NavigateUp, DeleteWordBackward]);

pub struct app {
    pub search_bar: Entity<SearchBar>,
    text: SharedString,
    _subscription: Vec<Subscription>,
    focus_handle: FocusHandle,

    engine: Arc<QueryEngine>,
    dispatcher: Arc<ActionDispatcher>,
    results: Vec<ResultItem>,
    selected_index: usize,
    is_searching: bool,
    scroll_handle: ScrollHandle,
}

impl app {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, engine: Arc<QueryEngine>, dispatcher: Arc<ActionDispatcher>) -> Self {
        let search_bar = cx.new(|_cx| SearchBar::new(_cx));
        search_bar.update(cx, |search_bar, cx| search_bar.input_state.focus_handle(cx).focus(window));
        
        let focus_handle = cx.focus_handle();
        
        cx.bind_keys([
            KeyBinding::new("escape", HideApp, None),
            KeyBinding::new("enter", ExecuteSelected, None),
            KeyBinding::new("down", NavigateDown, None),
            KeyBinding::new("up", NavigateUp, None),
            KeyBinding::new("tab", NavigateDown, None),
            KeyBinding::new("shift-tab", NavigateUp, None),
            KeyBinding::new("ctrl-backspace", DeleteWordBackward, None)
        ]);
        
        cx.observe_window_activation(window, |_this, window, cx| {
            if !window.is_window_active() {
                println!("Focus removed from application so hiding window.");
                window.hide_window();
            }
        }).detach();

        let _subscription = vec![cx.subscribe_in(&search_bar.read(cx).input_state.clone(), window, {
            let input_state = search_bar.read(cx).input_state.clone();
            move |this, _, ev: &InputEvent, _window, cx| match ev {
                InputEvent::Change => {
                    let value: String = input_state.read(cx).content().to_string();
                    this.text = value.into();
                    this.selected_index = 0;
                    let query = this.text.to_string();
                    let engine = this.engine.clone();
                    if query.is_empty() {
                        this.results.clear();
                        cx.notify();
                    } else {
                        this.is_searching = true;
                        cx.notify();

                        cx.spawn(async move |this, cx| {
                            let plugin_cx = PluginContext {config: serde_json::json!({})};
                            if let Ok(new_results) = engine.search(&query, &plugin_cx).await {
                                let _ = this.update(cx, |this, cx| {
                                    this.results = new_results;
                                    this.is_searching = false;
                                    cx.notify();
                                });
                            }
                        }).detach();
                    }
                }
                _ => {}
            }
        })];

        
        Self {
            search_bar,
            text: SharedString::default(),
            _subscription,
            focus_handle,

            engine,
            results: Vec::new(),
            selected_index: 0,
            is_searching: false,
            dispatcher,
            scroll_handle: ScrollHandle::new(),
        }
    }
    
    fn hide_app(&mut self, _: &HideApp, window: &mut Window, cx: &mut Context<Self>) {
        println!("Hiding app!!");
        window.hide_window();
    }
    fn execute_selected(&mut self, _: &ExecuteSelected, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(result) = self.results.get(self.selected_index) {
            let action = result.action.clone();
            let plugin_id = result.plugin_id.to_string();
            let context = PluginContext {config: serde_json::json!({})};
            self.dispatcher.execute(plugin_id, action, context);
            window.hide_window();
        }
    }

    fn navigate_down(&mut self, _: &NavigateDown, window: &mut Window, cx: &mut Context<Self>) {
        if !self.results.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.results.len();
            cx.notify();
            // scrolling to the item
            self.scroll_handle.scroll_to_item(self.selected_index);
        }
    }

    fn navigate_up(&mut self, _: &NavigateUp, window: &mut Window, cx: &mut Context<Self>) {
        if !self.results.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.results.len() - 1;
            } else {
                self.selected_index = (self.selected_index - 1) % self.results.len();
            }
            cx.notify();
            // scrolling to the item
            self.scroll_handle.scroll_to_item(self.selected_index);
        }
    }

    fn delete_word_backward(&mut self, _: &DeleteWordBackward, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.text.to_string();
        let mut words = text.split_whitespace().collect::<Vec<&str>>();
        if !words.is_empty() {
            words.pop();
            self.search_bar.update(cx, |state, cx| {
                self.text = words.join(" ").into();
            });
            cx.notify();
        }
    }
}

impl Render for app {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {

        let search_bar_height = 60.0_f32;  // search bar + margin
        let row_height = 60.0_f32;
        let padding = 16.0_f32;  // p_2 top + bottom
        
        let results_height = if self.results.is_empty() {
            0.0
        } else {
            let raw = (self.results.len() as f32 * row_height) + padding;
            raw.min(400.0 + padding)  // capped at max_h(400) + padding
        };
        
        let total_height = search_bar_height + results_height;
        
        // Resize the window dynamically
        window.resize(size(px(610.0), px(total_height)));

        div()
            .flex()
            .flex_col()
            .bg(rgb(0x13144A))
            .w(px(610.0))
            .p_0()
            .m_0()
            .gap_0()
            .justify_center()
            .items_center()
            .key_context("Input_element")            
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::hide_app))
            .on_action(cx.listener(Self::execute_selected))
            .on_action(cx.listener(Self::navigate_down))
            .on_action(cx.listener(Self::navigate_up))
            .on_action(cx.listener(Self::delete_word_backward))
            .child(
                Input::new(&self.search_bar.read(cx).input_state)
                    .placeholder("Type to search...")
                    .border_0()
                    .w(px(600.0))
                    .h(px(50.0))
                    .text_size(px(30.0))
                    .line_height(px(35.0))
                    .m_1()
                    .bg(rgb(0x13144A))
                    .variant(InputVariant::Ghost)
                    .justify_center()
            )
            .child(
                div()
                    .flex()
                    .p_2()
                    .flex_col()
                    .gap_1()
                    .child(
                        ResultList::new(self.results.clone(), self.selected_index, self.is_searching, self.scroll_handle.clone())
                    ) 
            )
    }
}
