# UltraSearch UI - GPUI API Fixes Required

## Summary

The UI code uses many styling methods that don't exist in GPUI core. This document outlines what needs to be fixed.

## Issues by File

### 1. **search_view.rs** - CRITICAL ISSUES

**Non-Existent Methods to Remove:**
- `.transition_colors()` - Line 96, 174
- `.transition_all()` - Line 155
- `.animate_pulse()` - Line 258
- `.placeholder()` on div - Lines 147-149
- `TextInput::new(cx)` - Lines 157-162

**Fix Strategy:**
1. Remove all transition/animation methods (nice-to-have, not critical)
2. Replace `TextInput` with proper gpui-component `Input` or simple div with text
3. Remove `.placeholder()` - just show text conditionally instead

### 2. **results_table.rs** - MODERATE ISSUES

**Non-Existent Methods to Remove:**
- `.text_ellipsis()` - Lines 211, 220
- `.whitespace_nowrap()` - Lines 212, 221
- `.text_align_right()` - Lines 245, 254
- `.text_transform_uppercase()` - Line 274
- `.letter_spacing()` - Line 275

**Fix Strategy:**
1. Remove text styling methods - GPUI doesn't support these CSS-like properties
2. Text will just wrap/overflow naturally (acceptable for MVP)
3. Alignment can be done with flexbox instead

### 3. **preview_view.rs** - MODERATE ISSUES

**Non-Existent Methods to Remove:**
- `.line_height()` - Lines 214, 221, 329
- `.text_transform_uppercase()` - Lines 163, 262, 286
- `.letter_spacing()` - Lines 164, 263, 287
- `.font_family()` - Line 301
- `.text_align_center()` - Line 327
- `.max_w()` - Line 328
- `.opacity()` - Line 315
- `.active()` - Line 136
- `.scale()` - Line 136

**Fix Strategy:**
1. Remove all advanced typography methods
2. `.opacity()` can be replaced with alpha channel in rgb: `rgb(0x...).opacity(0.3)` → `rgba(r, g, b, a)`
3. Remove active/scale animations
4. Remove font-family (will use system default)

### 4. **main.rs** - MINOR ISSUES

**Potential API Issues:**
- `cx.focus_view(&self.search_view)` - Line 62 - may not exist
- `self.search_view.focus_handle(cx)` - Line 65 - method signature might be wrong

**Fix Strategy:**
1. Check if focus API exists in current GPUI
2. May need to use different focus management approach

## Recommended Immediate Actions

### Priority 1: Make It Compile
1. Comment out or remove all non-existent styling methods
2. Replace TextInput with simple div showing text
3. Test basic rendering

### Priority 2: Add Basic Interactivity
1. Implement text input properly using gpui-component or custom solution
2. Add keyboard navigation without fancy animations
3. Mouse click handlers for row selection

### Priority 3: Polish (Future)
1. Investigate custom CSS-in-Rust solutions for text styling
2. Add animations using GPUI's animation system (if it exists)
3. Proper text input with cursor, selection, etc.

## Alternative Approaches

### Option A: Minimal Viable UI (Recommended)
- Strip out all fancy styling
- Use only basic GPUI methods
- Focus on functionality over aesthetics
- **Pros:** Will compile and work
- **Cons:** Less polished

### Option B: Use gpui-component Library
- Add `gpui-component` to dependencies
- Use their `Input`, `List`, etc. components
- Follow their documentation
- **Pros:** More features out of the box
- **Cons:** Another dependency, learning curve

### Option C: Custom Implementations
- Build our own text input component
- Implement text styling via custom rendering
- Full control over behavior
- **Pros:** Complete control
- **Cons:** Lots of work, reinventing wheel

## Immediate Next Steps

1. ✅ Run `cargo build` to see actual compilation errors
2. Create simplified versions of each view component
3. Test basic functionality without polish
4. Incrementally add features that work with GPUI

## References

- [GPUI Docs](https://docs.rs/gpui/latest/gpui/)
- [GPUI Component](https://longbridge.github.io/gpui-component/)
- [Zed Source Code](https://github.com/zed-industries/zed) - Best examples of GPUI usage
