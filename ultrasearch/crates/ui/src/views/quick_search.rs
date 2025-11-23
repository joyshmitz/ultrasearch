use gpui::*;
use crate::model::state::SearchAppModel;

pub struct QuickBarView {
    _model: Entity<SearchAppModel>,
    focus_handle: FocusHandle,
}

impl QuickBarView {
    pub fn new(model: Entity<SearchAppModel>, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self { _model: model, focus_handle }
    }

    pub fn focus_handle(&self) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for QuickBarView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(hsla(0.0, 0.0, 0.1, 0.95))
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.3, 1.0))
            .rounded_xl()
            .shadow_2xl()
            .flex()
            .items_center()
            .justify_center()
            .text_color(white())
            .child("Quick Search (Alt+Space)")
    }
}
