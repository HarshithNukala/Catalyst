use env_logger::fmt::style::Style;
use global_hotkey::hotkey::Modifiers;
use gpui:: {
    App, Application, AssetSource, AsyncApp, Bounds, Context, Entity, FocusHandle, Focusable, KeyBinding, KeyDownEvent, Point, ScrollHandle, SharedString, Size, Subscription, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, rgb, size
};
use std::result;
// use core::index;
use std::sync::{Arc, Condvar};
use adabraka_ui::components::input::{Input, InputEvent, InputVariant};
use adabraka_ui::components::input_state::InputState;

use crate::core::engine::{ActionDispatcher, QueryEngine};
use crate::core::model::{Action, BuiltInIcon, ResultIcon, ResultItem};
use crate::core::plugin;
use crate::core::plugin::{PluginContext, PluginRegistry};

actions!(Input_element, [HideApp, ExecuteSelected, NavigateDown, NavigateUp]);

pub struct Input_element {
    pub input_state: Entity<InputState>,
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

impl Input_element {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, engine: Arc<QueryEngine>, dispatcher: Arc<ActionDispatcher>) -> Self {
        let input_state = cx.new(|cx| InputState::new(cx));
        input_state.focus_handle(cx).focus(window);
        
        let focus_handle = cx.focus_handle();
        
        cx.bind_keys([
            KeyBinding::new("escape", HideApp, None),
            KeyBinding::new("enter", ExecuteSelected, None),
            KeyBinding::new("down", NavigateDown, None),
            KeyBinding::new("up", NavigateUp, None),
            KeyBinding::new("tab", NavigateDown, None),
            KeyBinding::new("shift-tab", NavigateUp, None),
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
            input_state,
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
}

impl Render for Input_element {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x505050))
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
            .child(
                Input::new(&self.input_state)
                    .placeholder("Type to search...")
                    .border_0()
                    .w(px(600.0))
                    .h(px(50.0))
                    .text_size(px(30.0))
                    .line_height(px(35.0))
                    .m_1()
                    // .bg(rgb(0x505050))
                    // .variant(InputVariant::Ghost)
                    // .border_color(rgb(0x505050)  )
            )
            .child(
                div()
                    .flex()
                    .border_2()
                    .border_color(gpui::white())
                    .p_2()
                    .flex_col()
                    .gap_1()
                    .child(
                        ResultList::new(self.results.clone(), self.selected_index, false, self.scroll_handle.clone())
                    ) 
            )
    }
}

#[derive(IntoElement)]
struct ResultRow {
    result: ResultItem,
    is_selected: bool,
}

impl ResultRow {
    pub fn new(result: ResultItem, is_selected: bool) -> Self {
        Self {
            result,
            is_selected,
        }
    }
}

impl RenderOnce for ResultRow {
    fn render(mut self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let result_id = self.result.id.clone();
        let is_selected = self.is_selected;
        let icon = self.result.icon.clone();
        let title = self.result.title.clone();
        let subtitle = self.result.subtitle.clone();

        let result_icon = |icon: &ResultIcon| -> gpui::AnyElement {
            match icon {
                ResultIcon::Path(_path) => {
                    div()
                        .size(px(40.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            title
                                .chars()
                                .next()
                                .unwrap_or('?')
                                .to_string()
                        )
                        .into_any_element()
                }
                ResultIcon::Emoji(emoji) => {
                    div()
                        .size(px(40.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(emoji.clone())
                        .into_any_element()
                }
                ResultIcon::AppIcon(_app_path) => {
                    div()
                        .size(px(40.0))
                        .rounded_md()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            title
                                .chars()
                                .next()
                                .unwrap_or('?')
                                .to_string()
                        )
                        .into_any_element()
                }
                ResultIcon::BuiltIn(builtin_icon) => {
                    let icon = match builtin_icon {
                        BuiltInIcon::Calculator => "🔢",
                        BuiltInIcon::Search => "🔍",
                        BuiltInIcon::File => "📄",
                        BuiltInIcon::Folder => "📁",
                        BuiltInIcon::Terminal => "💻",
                        BuiltInIcon::Settings => "⚙️",
                        BuiltInIcon::AI => "🤖",
                        BuiltInIcon::Web => "🌐",
                        BuiltInIcon::App => "📱"
                    };
                    div()
                        .size(px(40.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(icon)
                        .into_any_element()
                }
            }
        };

        div()
            .id(SharedString::from(result_id))
            .flex()
            .w_full()
            .h(px(60.0))
            .px_4()
            .items_center()
            .gap_3()
            .cursor_pointer()
            .when(is_selected, |this| {
                this.bg(gpui::rgb(0xe3f2fd))
            })
            .when(!is_selected, |this| {
                this.hover(|style| {
                    style.bg(gpui::rgb(0xe3f2fd))
                })
            })
            .child(result_icon(&icon))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .justify_center()
                    .overflow_hidden()
                    .child(
                        div()
                        .font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(gpui::white())
                            .truncate()
                            .child(title)
                            .when(is_selected, |this| {
                                this.text_color(gpui::black())
                            })
                    )
                    .when_some(subtitle, |this, subtitle| {
                        this.child(
                            div()
                                .text_color(gpui::rgb(0x666666))
                                .truncate()
                                .child(subtitle)
                        )
                    })
            )
    }
}

#[derive(IntoElement)]
struct ResultList {
    results: Vec<ResultItem>,
    selected_index: usize,
    is_searching: bool,
    scroll_handle: ScrollHandle,
}

impl ResultList {
    pub fn new(results: Vec<ResultItem>, selected_index: usize, is_searching: bool, scroll_handle: ScrollHandle) -> Self {
        Self { results, selected_index, is_searching, scroll_handle }
    }
}

impl RenderOnce for ResultList {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut list_div = 
            div()
                .id("result_list")
                // .flex_1()
                // .w_full()
                .w(px(600.0))
                .max_h(px(400.0))
                .bg(gpui::black())
                .border_1()
                .border_color(gpui::white())
                .overflow_y_scroll()
                .track_scroll(&self.scroll_handle);
        if self.is_searching {
            list_div = list_div.child(
                div()
                    .p_4()
                    .flex()
                    .justify_center()
                    .text_color(gpui::rgb(0x666666))
                    .child("Searching...")
            );
        } else if self.results.is_empty() {
            list_div = list_div.child(
                div()
                .p_8()
                .flex()
                .items_center()
                .gap_2()
                // .w_full()
                .w(px(600.0))
                .bg(gpui::black())
                .h(px(60.0))
                .rounded_md()
                .child(
                    div()
                    .text_size(px(48.0))
                    .child("🔍")
                )
                .child(
                    div()
                    .text_color(gpui::rgb(0x999999))
                    .child("No results found.")
                )
            );
        } else {
            for (index, result) in self.results.into_iter().enumerate() {
                list_div = list_div.child(
                    ResultRow::new(result, index == self.selected_index)
                );
            }
        }
        list_div
    }
}
