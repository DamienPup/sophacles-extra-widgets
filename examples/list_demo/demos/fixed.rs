use ratatui::{
    layout::Rect,
    style::{Color, Style},
    Frame,
};

use extra_widgets::styled_list::{ItemDisplay, StyledList, WindowType};

use super::super::{words, AppState};

pub fn fixed(area: Rect, state: &mut AppState, f: &mut Frame) {
    let demo_items = words();
    let demo_list = StyledList::new(demo_items)
        .default_style(Style::reset().bg(Color::Black).fg(Color::White))
        .selected_style(Style::default().bg(Color::Blue).fg(Color::White))
        .item_display(ItemDisplay::Basic)
        .window_type(WindowType::Fixed(3));

    f.render_stateful_widget(demo_list, area, &mut state.examples);
}
