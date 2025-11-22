# UltraSearch UI - Complete Fix Implementation Plan

## Executive Summary

This document contains ALL changes needed to get the UI compiling and working with GPUI.
Implementation time: ~2 hours if done systematically.

---

## Phase 1: Critical Compilation Fixes (15 min)

### 1.1 ✅ DONE: ModelContext → Context
Already fixed in 5 locations.

### 1.2 ✅ DONE: Color Types
Already converted Rgb → Hsla for constants.

### 1.3 TODO: Remaining rgb() Calls
Replace these inline rgb() calls:

**File: preview_view.rs**
```rust
// Line 122: Change
.hover(|style| style.bg(rgb(0x106ebe)))
// To:
.hover(|style| style.bg(hsla(207.0, 0.897, 0.556, 1.0)))

// Line 123: REMOVE (`.active()` and `.scale()` don't exist)
.active(|style| style.bg(rgb(0x005a9e)).scale(0.98))
// Replace with just the base styling (no active state for now)

// Line 139:
.bg(rgb(0x242424))
// To:
.bg(hsla(0.0, 0.0, 0.141, 1.0))
```

**File: results_table.rs**
```rust
// Line 223:
.bg(rgb(0x333333))
// To:
.bg(hsla(0.0, 0.0, 0.2, 1.0))

// Line 258:
.bg(rgb(0x242424))
// To:
.bg(hsla(0.0, 0.0, 0.141, 1.0))
```

**File: search_view.rs**
```rust
// Line 219:
.bg(rgb(0x242424))
// To:
.bg(hsla(0.0, 0.0, 0.141, 1.0))
```

**File: main.rs**
```rust
// Line 122:
.text_color(rgb(0xe4e4e4))
// To:
.text_color(hsla(0.0, 0.0, 0.894, 1.0))
```

---

## Phase 2: Remove Non-Existent Methods (30 min)

### 2.1 search_view.rs - Remove These Methods

**Lines to REMOVE:**
- Line 96: `.transition_colors(Duration::from_millis(150))`
- Line 155: `.transition_all(Duration::from_millis(150))`
- Line 174: `.transition_colors(Duration::from_millis(150))`
- Line 258: `.animate_pulse(Duration::from_secs(2))`
- Lines 147-149: `.placeholder()` on div
- Lines 157-162: `TextInput::new(cx)` component

**TextInput Replacement Strategy:**
For now, replace the entire TextInput section with a simple div showing the text:

```rust
// Replace lines 134-163 with:
.child(
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
        .when(cx.focused(&self.focus_handle), |this| {
            this.bg(INPUT_BG_FOCUS)
                .border_color(INPUT_BORDER_FOCUS)
                .shadow_md()
        })
        .child(if model.query.is_empty() {
            div().text_color(TEXT_PLACEHOLDER).child("Type to search...")
        } else {
            div().child(&model.query)
        })
)
```

### 2.2 results_table.rs - Remove Text Styling

**Lines to REMOVE:**
- Lines 211, 220: `.text_ellipsis()`
- Lines 212, 221: `.whitespace_nowrap()`
- Lines 245, 254: `.text_align_right()`
- Line 274: `.text_transform_uppercase()`
- Line 275: `.letter_spacing(px(0.5))`

Text will just wrap naturally - acceptable for MVP.

### 2.3 preview_view.rs - Remove Typography

**Lines to REMOVE:**
- Lines 214, 221, 329: `.line_height(relative(1.3))` etc.
- Lines 163, 262, 286: `.text_transform_uppercase()`
- Lines 164, 263, 287: `.letter_spacing(px(0.8))`
- Line 301: `.font_family("...")`
- Line 327: `.text_align_center()`
- Line 328: `.max_w(px(280.))`
- Line 315: `.opacity(0.3)` - Replace with `hsla(..., 0.3)` in color
- Lines 136, 123: `.active()` and `.scale()`

**Opacity Fix:**
```rust
// Line 315: Instead of .opacity(0.3)
// Change the child color directly:
.child(div().text_size(px(64.)).text_color(hsla(0.0, 0.0, 0.0, 0.3)).child("📄"))
```

### 2.4 main.rs - Focus API

**Potential Issue at Line 62:**
```rust
cx.focus_view(&self.search_view)
```

If this doesn't exist, replace with:
```rust
self.search_view.update(cx, |view, cx| {
    cx.focus(&view.focus_handle);
});
```

---

## Phase 3: gpui-component Integration (45 min)

### 3.1 Add Dependency

**File: ultrasearch/crates/ui/Cargo.toml**

Add after line 18:
```toml
gpui-component = { workspace = true }
```

### 3.2 Implement Proper Text Input

**File: search_view.rs**

Add import:
```rust
use gpui_component::{input::InputState, Input};
```

Update SearchView struct:
```rust
pub struct SearchView {
    model: Model<SearchAppModel>,
    focus_handle: FocusHandle,
    input_state: Entity<InputState>,  // ADD THIS
}
```

Update constructor:
```rust
pub fn new(model: Model<SearchAppModel>, cx: &mut ViewContext<Self>) -> Self {
    let focus_handle = cx.focus_handle();
    cx.focus(&focus_handle);
    cx.observe(&model, |_, _, cx| cx.notify()).detach();

    // Create input state
    let input_state = cx.new_entity(|cx| {
        InputState::new(cx)
            .placeholder("Search files by name or content...")
    });

    // Listen to input changes
    cx.subscribe(&input_state, |this: &mut Self, _, event, cx| {
        if let InputEvent::Change(text) = event {
            this.model.update(cx, |model, cx| {
                model.set_query(text.clone(), cx);
            });
        }
    }).detach();

    Self {
        model,
        focus_handle,
        input_state,
    }
}
```

Use in render:
```rust
.child(
    Input::new(cx)
        .state(self.input_state.clone())
        .class("search-input")
        .w_full()
)
```

### 3.3 Use VirtualList (Optional)

If gpui-component provides a better list component, use it for ResultsView.

---

## Phase 4: Restore Interactivity (30 min)

### 4.1 Row Click Handlers

**File: results_table.rs**

Make `render_row` non-static again and add handlers:

```rust
fn render_row(&self, index: usize, hit: &SearchHit, cx: &ViewContext<Self>) -> impl IntoElement {
    let is_selected = self.model.read(cx).is_selected(index);
    let is_hover = self.hover_index == Some(index);

    // ... existing rendering code ...

    div()
        // ... styles ...
        .on_mouse_enter(cx.listener(move |this, _, cx| {
            this.hover_index = Some(index);
            cx.notify();
        }))
        .on_mouse_leave(cx.listener(move |this, _, cx| {
            this.hover_index = None;
            cx.notify();
        }))
        .on_click(cx.listener(move |this, _, cx| {
            this.model.update(cx, |model, cx| {
                model.selected_index = Some(index);
                cx.notify();
            });
        }))
        .on_double_click(cx.listener(move |this, _, cx| {
            let model = this.model.read(cx);
            if let Some(hit) = model.results.get(index) {
                if let Some(path) = &hit.path {
                    this.open_file(path);
                }
            }
        }))
        // ... rest of row content ...
}
```

### 4.2 Keyboard Navigation

**File: main.rs - UltraSearchWindow::handle_key_down**

Already implemented! Just verify it works.

### 4.3 Enter to Open

Add to handle_key_down:
```rust
("enter", false, false) => {
    if let Some(hit) = self.model.read(cx).selected_row() {
        if let Some(path) = &hit.path {
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("explorer")
                    .arg(path)
                    .spawn()
                    .ok();
            }
        }
    }
    true
}
```

---

## Phase 5: Testing (20 min)

### 5.1 Compilation Test
```bash
cd ultrasearch
cargo build --release
```

### 5.2 Run UI
```bash
cargo run --bin search-ui
```

### 5.3 Manual Test Checklist
- [ ] Window opens
- [ ] Can type in search box
- [ ] Results appear
- [ ] Can select results with mouse
- [ ] Can navigate with arrow keys
- [ ] Enter opens file
- [ ] Mode buttons work
- [ ] Preview pane shows details

---

## Quick Reference: Common GPUI Methods That DO Exist

✅ **Layout:** `.flex()`, `.flex_col()`, `.items_center()`, `.justify_between()`
✅ **Spacing:** `.gap_3()`, `.px_4()`, `.py_2()`, `.p_6()`, `.m_2()`
✅ **Styling:** `.bg()`, `.text_color()`, `.border_1()`, `.rounded_md()`, `.shadow_sm()`
✅ **Sizing:** `.w_full()`, `.h(px())`, `.size_full()`, `.flex_1()`
✅ **Events:** `.on_click()`, `.on_mouse_enter()`, `.on_key_down()`
✅ **Conditional:** `.when()`, `.hover()`, `.child()`

❌ **Don't Exist:** `.transition_*()`, `.animate_*()`, `.text_ellipsis()`,
`.whitespace_nowrap()`, `.text_align_*()`, `.letter_spacing()`,
`.line_height()`, `.opacity()` (use alpha in color instead)

---

## Implementation Order

1. ✅ Phase 1.1-1.2 (DONE)
2. Phase 1.3: Fix remaining rgb() calls (5 minutes)
3. Phase 2: Remove all non-existent methods (25 minutes)
4. Phase 3.1: Add gpui-component dependency (1 minute)
5. Test compilation (5 minutes)
6. Phase 3.2: Implement proper text input (20 minutes)
7. Phase 4: Add interactivity (25 minutes)
8. Phase 5: Test everything (15 minutes)

**Total: ~2 hours**

---

## Next Step

Would you like me to:
1. **Apply all fixes automatically** (fastest, ~30 min of my time)
2. **Guide you through each phase** (educational, ~2 hours your time)
3. **Create a git branch with fixes** (you can review/merge)

Option 1 recommended for speed!
