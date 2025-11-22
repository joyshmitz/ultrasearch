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

const APP_BG: Hsla = hsla(0.0, 0.0, 0.102, 1.0);
const DIVIDER_COLOR: Hsla = hsla(0.0, 0.0, 0.2, 1.0);

/// Main application window containing all UI components
struct UltraSearchWindow {
    model: Model<SearchAppModel>,
    search_view: View<SearchView>,
    results_view: View<ResultsView>,
    preview_view: View<PreviewView>,
    focus_handle: FocusHandle,
}

impl UltraSearchWindow {
    fn new(cx: &mut WindowContext) -> Self {
        // Create the shared model
        let model = cx.new_model(|cx| {
            let mut app_model = SearchAppModel::new(cx);
            app_model.start_status_polling(cx);
            app_model
        });

        // Create all views with the shared model
        let search_view = cx.new_view(|cx| SearchView::new(model.clone(), cx));
        let results_view = cx.new_view(|cx| ResultsView::new(model.clone(), cx));
        let preview_view = cx.new_view(|cx| PreviewView::new(model.clone(), cx));

        let focus_handle = cx.focus_handle();

        Self {
            model,
            search_view,
            results_view,
            preview_view,
            focus_handle,
        }
    }

    fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut WindowContext) -> bool {
        let modifiers = &event.keystroke.modifiers;
        let key = &event.keystroke.key;

        // Global keyboard shortcuts
        match (
            key.as_str(),
            modifiers.command || modifiers.ctrl,
            modifiers.shift,
        ) {
            // Ctrl/Cmd+K: Focus search
            ("k", true, false) => {
                cx.focus_view(&self.search_view);
                true
            }
            // Escape: Clear search
            ("escape", false, false) => {
                self.model.update(cx, |model, cx| {
                    model.set_query(String::new(), cx);
                });
                true
            }
            // Up arrow: Previous result
            ("up", false, false) if !cx.focused(&self.search_view.focus_handle(cx)) => {
                self.model.update(cx, |model, cx| model.select_previous(cx));
                true
            }
            // Down arrow: Next result
            ("down", false, false) if !cx.focused(&self.search_view.focus_handle(cx)) => {
                self.model.update(cx, |model, cx| model.select_next(cx));
                true
            }
            // Ctrl/Cmd+1: Name-only mode
            ("1", true, false) => {
                self.model.update(cx, |model, cx| {
                    model.set_backend_mode(BackendMode::MetadataOnly, cx);
                });
                true
            }
            // Ctrl/Cmd+2: Mixed mode
            ("2", true, false) => {
                self.model.update(cx, |model, cx| {
                    model.set_backend_mode(BackendMode::Mixed, cx);
                });
                true
            }
            // Ctrl/Cmd+3: Content mode
            ("3", true, false) => {
                self.model.update(cx, |model, cx| {
                    model.set_backend_mode(BackendMode::ContentOnly, cx);
                });
                true
            }
            // Ctrl/Cmd+Q: Quit application
            ("q", true, false) => {
                cx.quit();
                true
            }
            _ => false,
        }
    }
}

impl Render for UltraSearchWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::handle_key_down))
            .size_full()
            .flex()
            .flex_col()
            .bg(APP_BG)
            .text_color(rgb(0xe4e4e4))
            .font_family("system-ui, -apple-system, 'Segoe UI', sans-serif")
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
                            .border_color(DIVIDER_COLOR)
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
    App::new().run(|cx: &mut AppContext| {
        // Register application metadata
        cx.set_app_id("com.ultrasearch.desktop");

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
                window_background: WindowBackgroundAppearance::Opaque,
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                is_movable: true,
                display_id: None,
            },
            |cx| cx.new_view(UltraSearchWindow::new),
        )
        .expect("Failed to open window");

        // Print startup message
        eprintln!("🔍 UltraSearch started successfully!");
        eprintln!("💡 Keyboard shortcuts:");
        eprintln!("   Ctrl+K        - Focus search");
        eprintln!("   Escape        - Clear search");
        eprintln!("   ↑/↓           - Navigate results");
        eprintln!("   Ctrl+1/2/3    - Switch search modes");
        eprintln!("   Ctrl+Q        - Quit");
    });
}
