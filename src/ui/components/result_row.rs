use gpui:: {
    App, Application, AssetSource, AsyncApp, Bounds, Context, Entity, FocusHandle, Focusable, KeyBinding, KeyDownEvent, Point, ScrollHandle, SharedString, Size, Subscription, Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, rgb, size
};
use crate::core::model::ResultItem;
use crate::core::model::ResultIcon;
use crate::core::model::BuiltInIcon;

#[derive(IntoElement)]
pub struct ResultRow {
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
                        BuiltInIcon::App => "📱",
                        BuiltInIcon::Dictionary => "📖",
                        BuiltInIcon::IP => "🌐",
                        BuiltInIcon::Exit => "❌",
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
            .rounded(px(5.0))
            .bg(rgb(0x07006C))
            .border_0()
            .when(is_selected, |this| {
                this.bg(gpui::rgb(0x1000A9))
            })
            .when(!is_selected, |this| {
                this.hover(|style| {
                    style.bg(gpui::rgb(0x1000A9))
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
                                this.text_color(gpui::white())
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