use gpui:: {
    App, Application, AssetSource, AsyncApp, Bounds, Context, Entity, FocusHandle, Focusable, KeyBinding, KeyDownEvent, Point, ScrollHandle, SharedString, Size, Subscription, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, rgb, size
};
use crate::core::model::ResultItem;
use super::result_row::ResultRow;

#[derive(IntoElement)]
pub struct ResultList {
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
        if self.results.is_empty() {
            return div()
                .id("empty");
        } 
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