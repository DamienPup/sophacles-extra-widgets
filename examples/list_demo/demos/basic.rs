use ratatui::{
    layout::Rect,
    style::{Color, Style},
    Frame,
};

use extra_widgets::styled_list::{ItemDisplay, StyledList};

use super::super::{words, AppState};

pub fn basic(area: Rect, state: &mut AppState, f: &mut Frame) {
    let demo_items = words();
    let demo_list = StyledList::new(demo_items)
        .default_style(Style::reset().bg(Color::Black).fg(Color::White))
        .selected_style(Style::default().bg(Color::Blue).fg(Color::White))
        .item_display(ItemDisplay::Basic);
    f.render_stateful_widget(demo_list, area, &mut state.examples);
}
