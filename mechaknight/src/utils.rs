pub fn rect_ratatui_to_bevy(rect: ratatui::layout::Rect) -> bevy::math::URect {
    bevy::math::URect::new(
        rect.left() as u32,
        rect.top() as u32,
        rect.right() as u32 - 1,
        rect.bottom() as u32 - 1,
    )
}
