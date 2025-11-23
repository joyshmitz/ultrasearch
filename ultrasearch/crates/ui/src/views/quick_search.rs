use crate::actions::{ClearSearch, OpenSelected, SelectNext, SelectPrev};
use crate::model::state::SearchAppModel;
use crate::theme;
use crate::views::results_table::ResultsView;
use crate::views::search_view::SearchView;
use gpui::*;

pub struct QuickBarView {
    search_view: Entity<SearchView>,
    results_view: Entity<ResultsView>,
    focus_handle: FocusHandle,
}

impl QuickBarView {
    pub fn new(model: Entity<SearchAppModel>, cx: &mut Context<Self>) -> Self {
        let search_view = cx.new(|cx| SearchView::new(model.clone(), cx));
        let results_view = cx.new(|cx| ResultsView::new(model.clone(), cx));

        // Use the search view's focus handle as the main handle for this view
        let focus_handle = search_view.read(cx).focus_handle();

        Self {
            search_view,
            results_view,
            focus_handle,
        }
    }

    pub fn focus_handle(&self) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for QuickBarView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = theme::active_colors(cx);

        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(hsla(0.0, 0.0, 0.0, 0.38))
            .key_context("QuickBar")
            .on_action(cx.listener(|_, _: &ClearSearch, window, _cx| {
                window.remove_window();
            }))
            .on_mouse_down_out(cx.listener(|_, _, window, _cx| {
                window.remove_window();
            }))
            .on_key_down(cx.listener(|_, event: &KeyDownEvent, window, cx| {
                match event.keystroke.key.as_str() {
                    "escape" => window.remove_window(),
                    "arrowdown" => cx.dispatch_action(&SelectNext),
                    "arrowup" => cx.dispatch_action(&SelectPrev),
                    "enter" => {
                        cx.dispatch_action(&OpenSelected);
                        window.remove_window();
                    }
                    _ => {}
                }
            }))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(900.))
                    .bg(colors.panel_bg)
                    .border_1()
                    .border_color(colors.match_highlight)
                    .rounded_xl()
                    .shadow_2xl()
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(div().flex_shrink_0().child(self.search_view.clone()))
                    .child(
                        div()
                            .flex_1()
                            .max_h(px(380.))
                            .overflow_hidden()
                            .child(self.results_view.clone()),
                    ),
            )
    }
}
