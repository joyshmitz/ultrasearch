use gpui::*;
use crate::model::state::{SearchAppModel, ResultRow, SortColumn};

pub struct ResultsView {
    model: Model<SearchAppModel>,
    list_state: ListState,
    focus_handle: FocusHandle,
}

impl ResultsView {
    pub fn new(model: Model<SearchAppModel>, cx: &mut ViewContext<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let view = cx.view().clone();
        
        let list_state = ListState::new(
            0,
            ListAlignment::Top,
            px(24.), // Item height
            move |ix, cx| {
                // We need to get the view back to access the model? 
                // ListState delegate captures context.
                // Actually, we can capture the model.
                view.update(cx, |this, cx| {
                    let model = this.model.read(cx);
                    if let Some(row) = model.results.get(ix) {
                        let selected = model.selected_index == Some(ix);
                        this.render_row(ix, row, selected, cx)
                    } else {
                        div().into_any_element()
                    }
                })
            },
        );

        // Observe model to update list count
        cx.observe(&model, |this: &mut Self, model, cx| {
            let count = model.read(cx).results.len();
            this.list_state.reset(count);
            
            // Scroll to selection if it changed? 
            // For now just ensure list count is correct.
            cx.notify();
        })
        .detach();

        Self {
            model,
            list_state,
            focus_handle,
        }
    }

    fn render_row(&self, ix: usize, row: &ResultRow, selected: bool, _cx: &mut ViewContext<Self>) -> AnyElement {
        let bg = if selected {
            rgb(0x2d2d2d) // Highlight
        } else if ix % 2 == 0 {
            rgb(0x1e1e1e) // Alt row
        } else {
            rgb(0x181818) // Base
        };

        div()
            .id(ix)
            .flex()
            .w_full()
            .h(px(24.))
            .bg(bg)
            .text_size(px(14.))
            .text_color(rgb(0xcccccc))
            .on_click(cx.listener(move |this, _, cx| {
                this.select_index(Some(ix), cx);
            }))
            .child(
                // Name
                div().w_4_12().px_2().overflow_hidden().child(row.name.clone())
            )
            .child(
                // Path
                div().w_5_12().px_2().overflow_hidden().child(row.path.clone())
            )
            .child(
                // Size (TODO: format)
                div().w_1_12().px_2().child(format!("{}", row.size))
            )
            .child(
                // Date (TODO: format)
                div().w_2_12().px_2().child(format!("{}", row.modified_ts))
            )
            .into_any_element()
    }

    fn select_index(&mut self, index: Option<usize>, cx: &mut ViewContext<Self>) {
        self.model.update(cx, |model, cx| {
            model.select_index(index, cx);
        });
        if let Some(ix) = index {
            self.list_state.scroll_to_reveal_item(ix);
        }
        cx.notify();
    }

    fn toggle_sort(&mut self, col: SortColumn, cx: &mut ViewContext<Self>) {
        self.model.update(cx, |model, cx| {
            model.set_sort(col, cx);
        });
    }

    fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut ViewContext<Self>) {
        let model = self.model.read(cx);
        let max = model.results.len();
        let current = model.selected_index;

        let next = match event.keystroke.key.as_str() {
            "up" => {
                if let Some(i) = current {
                    if i > 0 { Some(i - 1) } else { Some(0) }
                } else if max > 0 {
                    Some(max - 1)
                } else {
                    None
                }
            },
            "down" => {
                if let Some(i) = current {
                    if i < max.saturating_sub(1) { Some(i + 1) } else { Some(max.saturating_sub(1)) }
                } else if max > 0 {
                    Some(0)
                } else {
                    None
                }
            },
            _ => return,
        };

        if next != current {
            drop(model); // Release borrow
            self.select_index(next, cx);
        }
    }
}

impl Render for ResultsView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::handle_key_down))
            .size_full()
            .flex()
            .flex_col()
            .child(
                // Header
                div()
                    .flex()
                    .w_full()
                    .h(px(28.))
                    .bg(rgb(0x252526))
                    .border_b_1()
                    .border_color(rgb(0x333333))
                    .font_weight(FontWeight::BOLD)
                    .child(
                        div().w_4_12().px_2().child("Name")
                            .on_click(cx.listener(|this, _, cx| this.toggle_sort(SortColumn::Name, cx)))
                    )
                    .child(
                        div().w_5_12().px_2().child("Path")
                            .on_click(cx.listener(|this, _, cx| this.toggle_sort(SortColumn::Path, cx)))
                    )
                    .child(
                        div().w_1_12().px_2().child("Size")
                            .on_click(cx.listener(|this, _, cx| this.toggle_sort(SortColumn::Size, cx)))
                    )
                    .child(
                        div().w_2_12().px_2().child("Modified")
                            .on_click(cx.listener(|this, _, cx| this.toggle_sort(SortColumn::Modified, cx)))
                    )
            )
            .child(
                // List
                list(self.list_state.clone()).size_full()
            )
    }
}