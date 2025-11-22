use crate::model::state::SearchAppModel;
use gpui::prelude::*;
use gpui::*;
use ipc::SearchHit;
use std::process::Command;

const ROW_HEIGHT: Pixels = px(48.);
const TABLE_BG: Hsla = hsla(0.0, 0.0, 0.118, 1.0);
const ROW_EVEN: Hsla = hsla(0.0, 0.0, 0.118, 1.0);
const ROW_ODD: Hsla = hsla(0.0, 0.0, 0.141, 1.0);
const ROW_HOVER: Hsla = hsla(0.0, 0.0, 0.165, 1.0);
const ROW_SELECTED: Hsla = hsla(210.0, 0.274, 0.243, 1.0);
const ROW_SELECTED_HOVER: Hsla = hsla(210.0, 0.298, 0.294, 1.0);
const TEXT_PRIMARY: Hsla = hsla(0.0, 0.0, 0.894, 1.0);
const TEXT_SECONDARY: Hsla = hsla(0.0, 0.0, 0.616, 1.0);
const TEXT_DIM: Hsla = hsla(0.0, 0.0, 0.416, 1.0);
const BORDER_COLOR: Hsla = hsla(0.0, 0.0, 0.2, 1.0);

pub struct ResultsView {
    model: Model<SearchAppModel>,
    list_state: ListState,
    hover_index: Option<usize>,
}

impl ResultsView {
    pub fn new(model: Model<SearchAppModel>, cx: &mut ViewContext<Self>) -> Self {
        let list_state = ListState::new(0, ListAlignment::Top, ROW_HEIGHT);

        // Subscribe to model updates to refresh the list
        cx.observe(&model, |this: &mut Self, model, cx| {
            let count = model.read(cx).results.len();
            this.list_state.reset(count);
            cx.notify();
        })
        .detach();

        Self {
            model,
            list_state,
            hover_index: None,
        }
    }

    fn handle_click(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        self.model.update(cx, |model, cx| {
            model.selected_index = Some(index);
            cx.notify();
        });
    }

    fn handle_double_click(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        let model = self.model.read(cx);
        if let Some(hit) = model.results.get(index) {
            if let Some(path) = &hit.path {
                self.open_file(path);
            }
        }
    }

    fn open_file(&self, path: &str) {
        #[cfg(target_os = "windows")]
        {
            Command::new("explorer")
                .arg("/select,")
                .arg(path)
                .spawn()
                .ok();
        }
        #[cfg(target_os = "macos")]
        {
            Command::new("open").arg(path).spawn().ok();
        }
        #[cfg(target_os = "linux")]
        {
            Command::new("xdg-open").arg(path).spawn().ok();
        }
    }

    fn format_file_size(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if bytes >= GB {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.1} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.1} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} B", bytes)
        }
    }

    fn format_modified_time(timestamp: i64) -> String {
        use std::time::{Duration, SystemTime, UNIX_EPOCH};
        let now = SystemTime::now();
        let file_time = UNIX_EPOCH + Duration::from_secs(timestamp as u64);

        if let Ok(duration) = now.duration_since(file_time) {
            let days = duration.as_secs() / 86400;
            if days == 0 {
                "Today".to_string()
            } else if days == 1 {
                "Yesterday".to_string()
            } else if days < 7 {
                format!("{} days ago", days)
            } else if days < 30 {
                format!("{} weeks ago", days / 7)
            } else if days < 365 {
                format!("{} months ago", days / 30)
            } else {
                format!("{} years ago", days / 365)
            }
        } else {
            "Future".to_string()
        }
    }

    fn get_file_icon(ext: Option<&String>) -> &'static str {
        match ext.map(|s| s.as_str()) {
            Some("rs") | Some("toml") | Some("js") | Some("ts") | Some("tsx") | Some("jsx")
            | Some("py") | Some("go") => "📝",
            Some("pdf") => "📄",
            Some("docx") | Some("doc") => "📘",
            Some("xlsx") | Some("xls") => "📊",
            Some("pptx") | Some("ppt") => "📙",
            Some("zip") | Some("rar") | Some("7z") => "📦",
            Some("jpg") | Some("jpeg") | Some("png") | Some("gif") | Some("svg") => "🖼️",
            Some("mp4") | Some("avi") | Some("mkv") => "🎬",
            Some("mp3") | Some("wav") | Some("flac") => "🎵",
            Some("exe") | Some("dll") => "⚙️",
            Some("md") | Some("txt") => "📃",
            _ => "📄",
        }
    }

    fn render_row_static(
        index: usize,
        hit: &SearchHit,
        is_selected: bool,
        is_hover: bool,
    ) -> impl IntoElement {
        let is_even = index % 2 == 0;

        let name = hit.name.as_deref().unwrap_or("<unknown>");
        let path = hit.path.as_deref().unwrap_or("");
        let size_text = hit
            .size
            .map(Self::format_file_size)
            .unwrap_or_else(|| "-".to_string());
        let modified_text = hit
            .modified
            .map(Self::format_modified_time)
            .unwrap_or_else(|| "-".to_string());
        let icon = Self::get_file_icon(hit.ext.as_ref());
        let score_pct = (hit.score * 100.0) as u32;

        div()
            .w_full()
            .h(ROW_HEIGHT)
            .flex()
            .items_center()
            .px_4()
            .gap_3()
            .when(is_selected, |this| {
                this.bg(if is_hover {
                    ROW_SELECTED_HOVER
                } else {
                    ROW_SELECTED
                })
            })
            .when(!is_selected, |this| {
                this.bg(if is_hover {
                    ROW_HOVER
                } else if is_even {
                    ROW_EVEN
                } else {
                    ROW_ODD
                })
            })
            .border_b_1()
            .border_color(BORDER_COLOR)
            .cursor_pointer()
            // TODO: Add mouse event handlers (requires non-static method)
            // File icon
            .child(div().text_size(px(20.)).child(icon))
            // Name column (flexible)
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .overflow_hidden()
                    .child(
                        div()
                            .text_size(px(14.))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(TEXT_PRIMARY)
                            .overflow_hidden()
                            .child(name),
                    )
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(TEXT_SECONDARY)
                            .overflow_hidden()
                            .child(path),
                    ),
            )
            // Score badge
            .when(score_pct > 0, |this| {
                this.child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded_md()
                        .bg(hsla(0.0, 0.0, 0.2, 1.0))
                        .text_size(px(10.))
                        .font_weight(FontWeight::BOLD)
                        .text_color(TEXT_DIM)
                        .child(format!("{}%", score_pct)),
                )
            })
            // Size column
            .child(
                div()
                    .w(px(80.))
                    .text_size(px(12.))
                    .text_color(TEXT_SECONDARY)
                    .child(size_text),
            )
            // Modified column
            .child(
                div()
                    .w(px(100.))
                    .text_size(px(12.))
                    .text_color(TEXT_SECONDARY)
                    .child(modified_text),
            )
    }

    fn render_header(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h(px(40.))
            .flex()
            .items_center()
            .px_4()
            .gap_3()
            .bg(hsla(0.0, 0.0, 0.141, 1.0))
            .border_b_1()
            .border_color(BORDER_COLOR)
            .text_size(px(11.))
            .font_weight(FontWeight::BOLD)
            .text_color(TEXT_DIM)
            .child(div().w(px(20.))) // Icon space
            .child(div().flex_1().child("NAME"))
            .child(div().w(px(80.)).child("SIZE"))
            .child(div().w(px(100.)).child("MODIFIED"))
    }

    fn render_empty_state(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let model = self.model.read(cx);
        let has_query = !model.query.is_empty();

        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                div()
                    .text_size(px(48.))
                    .child(if has_query { "🔍" } else { "💾" }),
            )
            .child(
                div()
                    .text_size(px(16.))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(TEXT_SECONDARY)
                    .child(if has_query {
                        "No results found"
                    } else {
                        "Start typing to search"
                    }),
            )
            .when(has_query, |this| {
                this.child(
                    div()
                        .text_size(px(13.))
                        .text_color(TEXT_DIM)
                        .child("Try different search terms or search mode"),
                )
            })
    }
}

impl Render for ResultsView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let model = self.model.clone();
        let has_results = !model.read(cx).results.is_empty();
        let hover_index = self.hover_index;

        div()
            .size_full()
            .bg(TABLE_BG)
            .flex()
            .flex_col()
            .when(has_results, |this| {
                this.child(self.render_header(cx)).child(
                    list(self.list_state.clone(), move |ix, _window, cx| {
                        let model_read = model.read(cx);
                        if let Some(hit) = model_read.results.get(ix) {
                            let is_selected = model_read.is_selected(ix);
                            let is_hover = hover_index == Some(ix);
                            Self::render_row_static(ix, hit, is_selected, is_hover)
                                .into_any_element()
                        } else {
                            div().into_any_element()
                        }
                    })
                    .size_full()
                    .track_scroll(self.list_state.clone()),
                )
            })
            .when(!has_results, |this| this.child(self.render_empty_state(cx)))
    }
}
