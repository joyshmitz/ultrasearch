use crate::model::state::{BackendMode, SearchAppModel};
use gpui::prelude::*;
use gpui::*;

const SEARCH_BG: Hsla = hsla(0.0, 0.0, 0.102, 1.0);
const SEARCH_BORDER: Hsla = hsla(0.0, 0.0, 0.243, 1.0);
const INPUT_BG: Hsla = hsla(0.0, 0.0, 0.176, 1.0);
const INPUT_BG_FOCUS: Hsla = hsla(0.0, 0.0, 0.208, 1.0);
const INPUT_BORDER_FOCUS: Hsla = hsla(207.0, 1.0, 0.404, 1.0);
const TEXT_PRIMARY: Hsla = hsla(0.0, 0.0, 0.894, 1.0);
const TEXT_SECONDARY: Hsla = hsla(0.0, 0.0, 0.616, 1.0);
const TEXT_PLACEHOLDER: Hsla = hsla(0.0, 0.0, 0.416, 1.0);
const ACCENT_BLUE: Hsla = hsla(207.0, 1.0, 0.416, 1.0);
const STATUS_SUCCESS: Hsla = hsla(146.0, 0.444, 0.502, 1.0);
const STATUS_ERROR: Hsla = hsla(0.0, 0.903, 0.661, 1.0);

pub struct SearchView {
    model: Model<SearchAppModel>,
    focus_handle: FocusHandle,
    input_text: SharedString,
}

impl SearchView {
    pub fn new(model: Model<SearchAppModel>, cx: &mut ViewContext<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        // Auto-focus on creation for instant search experience
        cx.focus(&focus_handle);

        // Observe model changes
        cx.observe(&model, |_, _, cx| cx.notify()).detach();

        Self {
            model,
            focus_handle,
            input_text: "".into(),
        }
    }

    fn handle_input(&mut self, text: &str, cx: &mut ViewContext<Self>) {
        self.input_text = text.into();
        self.model.update(cx, |model, cx| {
            model.set_query(text.to_string(), cx);
        });
    }

    fn clear_search(&mut self, cx: &mut ViewContext<Self>) {
        self.input_text = "".into();
        self.model.update(cx, |model, cx| {
            model.set_query(String::new(), cx);
        });
    }

    fn set_mode(&mut self, mode: BackendMode, cx: &mut ViewContext<Self>) {
        self.model.update(cx, |model, cx| {
            model.set_backend_mode(mode, cx);
        });
    }

    fn format_number(n: u64) -> String {
        if n >= 1_000_000 {
            format!("{:.1}M", n as f64 / 1_000_000.0)
        } else if n >= 1_000 {
            format!("{:.1}K", n as f64 / 1_000.0)
        } else {
            n.to_string()
        }
    }

    fn render_mode_button(
        &self,
        label: &'static str,
        icon: &'static str,
        mode: BackendMode,
        current: BackendMode,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let is_active = mode == current;

        div()
            .flex()
            .items_center()
            .gap_1p5()
            .px_3()
            .py_1p5()
            .rounded_md()
            .when(is_active, |this| {
                this.bg(ACCENT_BLUE).text_color(white()).shadow_sm()
            })
            .when(!is_active, |this| {
                this.bg(INPUT_BG)
                    .text_color(TEXT_SECONDARY)
                    .hover(|style| style.bg(INPUT_BG_FOCUS).text_color(TEXT_PRIMARY))
            })
            .cursor_pointer()
            .transition_colors(Duration::from_millis(150))
            .child(div().text_size(px(14.)).child(icon))
            .child(
                div()
                    .text_size(px(13.))
                    .font_weight(FontWeight::MEDIUM)
                    .child(label),
            )
            .on_click(cx.listener(move |this, _, cx| this.set_mode(mode, cx)))
    }
}

impl Render for SearchView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let model = self.model.read(cx);
        let status = &model.status;
        let has_query = !model.query.is_empty();

        div()
            .flex()
            .flex_col()
            .w_full()
            .bg(SEARCH_BG)
            .border_b_1()
            .border_color(SEARCH_BORDER)
            .shadow_sm()
            .child(
                // Search input area with modern styling
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .px_4()
                    .py_3()
                    .child(
                        // Search icon
                        div()
                            .text_size(px(20.))
                            .text_color(TEXT_SECONDARY)
                            .child("🔍"),
                    )
                    .child(
                        // Text input with focus ring
                        div()
                            .id("search-input")
                            .flex_1()
                            .px_3()
                            .py_2p5()
                            .bg(INPUT_BG)
                            .border_1()
                            .border_color(SEARCH_BORDER)
                            .rounded_lg()
                            .text_color(TEXT_PRIMARY)
                            .text_size(px(15.))
                            .placeholder("Search files by name or content...", |style| {
                                style.text_color(TEXT_PLACEHOLDER)
                            })
                            .when(cx.focused(&self.focus_handle), |this| {
                                this.bg(INPUT_BG_FOCUS)
                                    .border_color(INPUT_BORDER_FOCUS)
                                    .shadow_md()
                            })
                            .transition_all(Duration::from_millis(150))
                            .child(
                                TextInput::new(cx)
                                    .text(self.input_text.clone())
                                    .on_input(cx.listener(Self::handle_input))
                                    .placeholder("Search files by name or content...")
                                    .font_size(px(15.)),
                            ),
                    )
                    .when(has_query, |this| {
                        this.child(
                            // Clear button (only shown when there's text)
                            div()
                                .px_2()
                                .py_1p5()
                                .rounded_md()
                                .text_color(TEXT_SECONDARY)
                                .hover(|style| style.bg(INPUT_BG_FOCUS).text_color(TEXT_PRIMARY))
                                .cursor_pointer()
                                .transition_colors(Duration::from_millis(150))
                                .child("✕")
                                .on_click(cx.listener(|this, _, cx| this.clear_search(cx))),
                        )
                    })
                    .child(
                        // Mode selector buttons
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(self.render_mode_button(
                                "Name",
                                "📄",
                                BackendMode::MetadataOnly,
                                status.backend_mode,
                                cx,
                            ))
                            .child(self.render_mode_button(
                                "Mixed",
                                "⚡",
                                BackendMode::Mixed,
                                status.backend_mode,
                                cx,
                            ))
                            .child(self.render_mode_button(
                                "Content",
                                "📝",
                                BackendMode::ContentOnly,
                                status.backend_mode,
                                cx,
                            )),
                    ),
            )
            .child(
                // Status bar with elegant information display
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .py_2()
                    .bg(rgb(0x242424))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .text_size(px(12.))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1p5()
                                    .child(div().text_color(TEXT_SECONDARY).child(format!(
                                        "{} results",
                                        Self::format_number(status.total)
                                    )))
                                    .when(status.shown < status.total as usize, |this| {
                                        this.child(
                                            div()
                                                .text_color(TEXT_PLACEHOLDER)
                                                .child(format!("(showing {})", status.shown)),
                                        )
                                    }),
                            )
                            .when(status.last_latency_ms.is_some(), |this| {
                                this.child(div().text_color(SEARCH_BORDER).child("•"))
                                    .child(
                                        div().text_color(TEXT_SECONDARY).child(format!(
                                            "{} ms",
                                            status.last_latency_ms.unwrap()
                                        )),
                                    )
                            })
                            .child(div().text_color(SEARCH_BORDER).child("•"))
                            .child(
                                div()
                                    .text_color(TEXT_SECONDARY)
                                    .child(&status.indexing_state),
                            ),
                    )
                    .child(
                        // Connection status with animated indicator
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .w(px(8.))
                                    .h(px(8.))
                                    .rounded_full()
                                    .bg(if status.connected {
                                        STATUS_SUCCESS
                                    } else {
                                        STATUS_ERROR
                                    })
                                    .when(status.connected, |this| {
                                        this.animate_pulse(Duration::from_secs(2))
                                    }),
                            )
                            .child(
                                div()
                                    .text_size(px(12.))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(if status.connected {
                                        STATUS_SUCCESS
                                    } else {
                                        STATUS_ERROR
                                    })
                                    .child(if status.connected {
                                        "Connected"
                                    } else {
                                        "Disconnected"
                                    }),
                            ),
                    ),
            )
    }
}
