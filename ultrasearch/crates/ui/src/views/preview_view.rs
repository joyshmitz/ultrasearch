use crate::model::state::SearchAppModel;
use gpui::prelude::*;
use gpui::*;
use std::process::Command;
use std::time::Duration;

const PREVIEW_BG: Hsla = hsla(0.0, 0.0, 0.102, 1.0);
const PREVIEW_BORDER: Hsla = hsla(0.0, 0.0, 0.2, 1.0);
const TEXT_PRIMARY: Hsla = hsla(0.0, 0.0, 0.894, 1.0);
const TEXT_SECONDARY: Hsla = hsla(0.0, 0.0, 0.616, 1.0);
const TEXT_DIM: Hsla = hsla(0.0, 0.0, 0.416, 1.0);
const ACCENT_BLUE: Hsla = hsla(207.0, 1.0, 0.416, 1.0);
const SNIPPET_BG: Hsla = hsla(0.0, 0.0, 0.157, 1.0);
const SNIPPET_BORDER: Hsla = hsla(0.0, 0.0, 0.243, 1.0);

pub struct PreviewView {
    model: Model<SearchAppModel>,
}

impl PreviewView {
    pub fn new(model: Model<SearchAppModel>, cx: &mut ViewContext<Self>) -> Self {
        cx.observe(&model, |_, _, cx| cx.notify()).detach();
        Self { model }
    }

    fn open_in_explorer(&mut self, path: &str, _cx: &mut ViewContext<Self>) {
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
            Command::new("open").arg("-R").arg(path).spawn().ok();
        }
        #[cfg(target_os = "linux")]
        {
            if let Some(parent) = std::path::Path::new(path).parent() {
                Command::new("xdg-open").arg(parent).spawn().ok();
            }
        }
    }

    fn open_file(&mut self, path: &str, _cx: &mut ViewContext<Self>) {
        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(["/C", "start", "", path])
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
        const TB: u64 = GB * 1024;

        if bytes >= TB {
            format!("{:.2} TB", bytes as f64 / TB as f64)
        } else if bytes >= GB {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.1} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.1} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} bytes", bytes)
        }
    }

    fn format_modified_time(timestamp: i64) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let datetime = UNIX_EPOCH + std::time::Duration::from_secs(timestamp as u64);

        // Format as "Jan 15, 2024 at 3:45 PM"
        // For simplicity, using basic formatting
        if let Ok(duration) = SystemTime::now().duration_since(datetime) {
            let days = duration.as_secs() / 86400;
            if days == 0 {
                "Today".to_string()
            } else if days == 1 {
                "Yesterday".to_string()
            } else {
                format!("{} days ago", days)
            }
        } else {
            "In the future".to_string()
        }
    }

    fn render_action_button(
        &self,
        icon: &'static str,
        label: &'static str,
        on_click: impl Fn(&mut Self, &ClickEvent, &mut ViewContext<Self>) + 'static,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            .px_4()
            .py_2p5()
            .bg(ACCENT_BLUE)
            .rounded_lg()
            .text_color(white())
            .font_weight(FontWeight::MEDIUM)
            .text_size(px(13.))
            .cursor_pointer()
            .hover(|style| style.bg(hsla(207.0, 0.897, 0.556, 1.0)))
            .shadow_md()
            .child(div().text_size(px(16.)).child(icon))
            .child(label)
            .on_click(cx.listener(on_click))
    }

    fn render_info_row(&self, label: &str, value: String, icon: &'static str) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_3()
            .px_4()
            .py_3()
            .rounded_lg()
            .bg(hsla(0.0, 0.0, 0.141, 1.0))
            .child(div().text_size(px(18.)).child(icon))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_0p5()
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(TEXT_DIM)
                            .child(label.to_uppercase()),
                    )
                    .child(
                        div()
                            .text_size(px(14.))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(TEXT_PRIMARY)
                            .child(value),
                    ),
            )
    }
}

impl Render for PreviewView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let model = self.model.read(cx);
        let selected = model.selected_row();

        let content = if let Some(hit) = selected {
            let name = hit.name.as_deref().unwrap_or("<unknown>").to_string();
            let path = hit.path.clone().unwrap_or_default();
            let size_text = hit
                .size
                .map(Self::format_file_size)
                .unwrap_or_else(|| "Unknown".to_string());
            let modified_text = hit
                .modified
                .map(Self::format_modified_time)
                .unwrap_or_else(|| "Unknown".to_string());
            let ext = hit.ext.clone().unwrap_or_else(|| "None".to_string());
            let score = format!("{:.1}%", hit.score * 100.0);

            div()
                .flex()
                .flex_col()
                .size_full()
                .overflow_y_scroll()
                .child(
                    // Header section with file name and actions
                    div()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .p_6()
                        .border_b_1()
                        .border_color(PREVIEW_BORDER)
                        .child(
                            div()
                                .text_size(px(20.))
                                .font_weight(FontWeight::BOLD)
                                .text_color(TEXT_PRIMARY)
                                .child(name.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(TEXT_SECONDARY)
                                .child(path.clone()),
                        )
                        .child(
                            // Action buttons
                            div()
                                .flex()
                                .gap_3()
                                .mt_2()
                                .child(self.render_action_button(
                                    "📂",
                                    "Open",
                                    {
                                        let path = path.clone();
                                        move |this, _, cx| this.open_file(&path, cx)
                                    },
                                    cx,
                                ))
                                .child(self.render_action_button(
                                    "📁",
                                    "Show in Folder",
                                    {
                                        let path = path.clone();
                                        move |this, _, cx| this.open_in_explorer(&path, cx)
                                    },
                                    cx,
                                )),
                        ),
                )
                .child(
                    // Details section
                    div()
                        .flex()
                        .flex_col()
                        .gap_3()
                        .p_6()
                        .child(
                            div()
                                .text_size(px(13.))
                                .font_weight(FontWeight::BOLD)
                                .text_color(TEXT_DIM)
                                .mb_3()
                                .child("FILE DETAILS"),
                        )
                        .child(self.render_info_row("Size", size_text, "💾"))
                        .child(self.render_info_row("Modified", modified_text, "📅"))
                        .child(self.render_info_row("Extension", ext.to_uppercase(), "🏷️"))
                        .child(self.render_info_row("Match Score", score, "⭐")),
                )
                .when(hit.snippet.is_some(), |this| {
                    this.child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .p_6()
                            .border_t_1()
                            .border_color(PREVIEW_BORDER)
                            .child(
                                div()
                                    .text_size(px(13.))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(TEXT_DIM)
                                    .mb_2()
                                    .child("CONTENT PREVIEW"),
                            )
                            .child(
                                div()
                                    .p_4()
                                    .bg(SNIPPET_BG)
                                    .border_1()
                                    .border_color(SNIPPET_BORDER)
                                    .rounded_lg()
                                    .text_size(px(12.))
                                    .text_color(TEXT_SECONDARY)
                                    .child(hit.snippet.as_ref().unwrap().clone()),
                            ),
                    )
                })
        } else {
            // Empty state
            div()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .size_full()
                .gap_4()
                .child(div().text_size(px(64.)).opacity(0.3).child("📄"))
                .child(
                    div()
                        .text_size(px(16.))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(TEXT_SECONDARY)
                        .child("No file selected"),
                )
                .child(
                    div()
                        .text_size(px(13.))
                        .text_color(TEXT_DIM)
                        .child("Select a file from the results to see details and preview"),
                )
        };

        div()
            .size_full()
            .bg(PREVIEW_BG)
            .border_l_1()
            .border_color(PREVIEW_BORDER)
            .child(content)
    }
}
