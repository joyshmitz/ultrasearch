//! UltraSearch - World-class desktop search application
//!
//! A high-performance Windows desktop search engine combining instant filename
//! search with deep content indexing, wrapped in a beautiful native UI.

use gpui::prelude::*;
use gpui::*;
use ui::model::state::{BackendMode, SearchAppModel};
use ui::views::preview_view::PreviewView;
use ui::views::results_table::ResultsView;
use ui::views::search_view::SearchView;

fn app_bg() -> Hsla {
    hsla(0.0, 0.0, 0.102, 1.0)
}
fn divider_color() -> Hsla {
    hsla(0.0, 0.0, 0.2, 1.0)
}
fn text_primary() -> Hsla {
    hsla(0.0, 0.0, 0.894, 1.0)
}

/// Main application window containing all UI components
struct UltraSearchWindow {
    model: Entity<SearchAppModel>,
    search_view: Entity<SearchView>,
    results_view: Entity<ResultsView>,
    preview_view: Entity<PreviewView>,
    focus_handle: FocusHandle,
}

impl UltraSearchWindow {
    fn new(cx: &mut Context<Self>) -> Self {
        let model = cx.new(SearchAppModel::new);

        let search_view = cx.new(|cx| SearchView::new(model.clone(), cx));
        let results_view = cx.new(|cx| ResultsView::new(model.clone(), cx));
        let preview_view = cx.new(|cx| PreviewView::new(model.clone(), cx));

        let focus_handle = cx.focus_handle();

        Self {
            model,
            search_view,
            results_view,
            preview_view,
            focus_handle,
        }
    }

    fn handle_key_down(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let modifiers = &event.keystroke.modifiers;
        let key = &event.keystroke.key;

        match (
            key.as_str(),
            modifiers.platform || modifiers.control,
            modifiers.shift,
        ) {
            // Ctrl/Cmd+K: Focus search
            ("k", true, false) => {
                self.search_view.update(cx, |view, _cx| {
                    window.focus(&view.focus_handle());
                });
                cx.stop_propagation();
            }
            // Escape: Clear search
            ("escape", false, false) => {
                self.model.update(cx, |model, cx| {
                    model.set_query(String::new(), cx);
                });
                cx.stop_propagation();
            }
            // Up arrow: Previous result
            ("up", false, false) => {
                self.model.update(cx, |model, cx| model.select_previous(cx));
                cx.stop_propagation();
            }
            // Down arrow: Next result
            ("down", false, false) => {
                self.model.update(cx, |model, cx| model.select_next(cx));
                cx.stop_propagation();
            }
            // Enter: open selected file
            ("enter", false, false) => {
                if let Some(path) = self
                    .model
                    .read(cx)
                    .selected_row()
                    .and_then(|hit| hit.path.clone())
                {
                    #[cfg(target_os = "windows")]
                    {
                        let _ = std::process::Command::new("explorer").arg(&path).spawn();
                    }
                    #[cfg(target_os = "macos")]
                    {
                        let _ = std::process::Command::new("open").arg(&path).spawn();
                    }
                    #[cfg(target_os = "linux")]
                    {
                        let _ = std::process::Command::new("xdg-open").arg(&path).spawn();
                    }
                }
                cx.stop_propagation();
            }
            // Ctrl/Cmd+1: Name-only mode
            ("1", true, false) => {
                self.model.update(cx, |model, cx| {
                    model.set_backend_mode(BackendMode::MetadataOnly, cx);
                });
                cx.stop_propagation();
            }
            // Ctrl/Cmd+2: Mixed mode
            ("2", true, false) => {
                self.model.update(cx, |model, cx| {
                    model.set_backend_mode(BackendMode::Mixed, cx);
                });
                cx.stop_propagation();
            }
            // Ctrl/Cmd+3: Content mode
            ("3", true, false) => {
                self.model.update(cx, |model, cx| {
                    model.set_backend_mode(BackendMode::ContentOnly, cx);
                });
                cx.stop_propagation();
            }
            // Ctrl/Cmd+Q: Quit application
            ("q", true, false) => {
                cx.quit();
            }
            _ => {}
        }
    }
}

impl Render for UltraSearchWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::handle_key_down))
            .size_full()
            .flex()
            .flex_col()
            .bg(app_bg())
            .text_color(text_primary())
            .child(
                // Search header - fixed at top
                div().flex_shrink_0().child(self.search_view.clone()),
            )
            .child(
                // Main content area - flexible height
                div()
                    .flex_1()
                    .flex()
                    .overflow_hidden()
                    .child(
                        // Results table - 60% width
                        div()
                            .flex_basis(relative(0.6))
                            .flex_grow()
                            .overflow_hidden()
                            .border_r_1()
                            .border_color(divider_color())
                            .child(self.results_view.clone()),
                    )
                    .child(
                        // Preview pane - 40% width
                        div()
                            .flex_basis(relative(0.4))
                            .flex_shrink_0()
                            .overflow_hidden()
                            .child(self.preview_view.clone()),
                    ),
            )
    }
}

fn main() {
    // Load configuration
    if let Err(e) = core_types::config::load_or_create_config(None) {
        eprintln!("Failed to load configuration: {}", e);
        eprintln!("Continuing with default configuration...");
    }

    // Initialize GPUI application
    Application::new().run(|cx: &mut App| {
        // Open the main window
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Point {
                        x: px(100.0),
                        y: px(100.0),
                    },
                    size: Size {
                        width: px(1400.0),
                        height: px(900.0),
                    },
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from("UltraSearch")),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                window_min_size: Some(Size {
                    width: px(960.0),
                    height: px(600.0),
                }),
                window_background: WindowBackgroundAppearance::Opaque,
                app_id: Some("com.ultrasearch.desktop".to_string()),
                ..WindowOptions::default()
            },
            |_, cx| cx.new(UltraSearchWindow::new),
        )
        .expect("Failed to open window");

        // Print startup message
        eprintln!("✅ UltraSearch started successfully!");
        eprintln!("🌀 Keyboard shortcuts:");
        eprintln!("   Ctrl+K        - Focus search");
        eprintln!("   Escape        - Clear search");
        eprintln!("   ↑/↓           - Navigate results");
        eprintln!("   Ctrl+1/2/3    - Switch search modes");
        eprintln!("   Ctrl+Q        - Quit");

        cx.activate(true);
    });
}
