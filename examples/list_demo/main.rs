use std::{error::Error, io};

use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

use widgets::{
    event::{Config, Event, Events},
    widgets::{
        separated_list::{ItemDisplay, WindowType},
        ListItem, ListState, SeparatedList,
    },
};

static WORDS: &str = include_str!("../wordlist.txt");

fn words<'a>() -> Vec<ListItem<'a>> {
    WORDS.trim_end().split('\n').map(ListItem::new).collect()
}

#[derive(Debug)]
enum Focus {
    Picker,
    Example,
}

impl Focus {
    fn toggle(&mut self) {
        use Focus::*;
        match self {
            Picker => *self = Example,
            Example => *self = Picker,
        }
    }
}

#[derive(Debug)]
struct AppState {
    focus: Focus,
    picker: ListState,
    examples: ListState,
}

impl AppState {
    fn new(n_picker: usize, n_examples: usize) -> Self {
        Self {
            focus: Focus::Picker,
            picker: ListState::new(n_picker),
            examples: ListState::new(n_examples),
        }
    }

    fn switch_focus(&mut self) {
        self.focus.toggle();
    }

    fn move_up(&mut self) {
        match self.focus {
            Focus::Picker => self.picker.cycle_prev(),
            Focus::Example => self.examples.cycle_prev(),
        }
    }

    fn move_down(&mut self) {
        match self.focus {
            Focus::Picker => self.picker.cycle_next(),
            Focus::Example => self.examples.cycle_next(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::with_config(Config::without_ticker());

    let mut state = AppState::new(1, words().len());

    loop {
        let mstate = &mut state;
        let _ = terminal.draw(|f| draw(mstate, f));

        match events.next()? {
            Event::Input(Key::Char(c)) if c == 'j' => {
                state.move_down();
            }
            Event::Input(Key::Char(c)) if c == 'k' => {
                state.move_up();
            }
            Event::Input(Key::Char(c)) if c == 'h' || c == 'l' => {
                state.switch_focus();
            }
            Event::Input(Key::Char(_)) => {
                break;
            }
            _ => {}
        };
    }
    Ok(())
}

fn draw<B: Backend>(state: &mut AppState, f: &mut Frame<B>) {
    let app_area = f.size();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Min(1),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        );

    let chunks = layout.split(app_area);
    //println!("{:?}", chunks);
    let bar_area = chunks[0];
    // this could be done other ways, i just like how it looks
    let border_area = chunks[1];
    let main_area = chunks[2];

    // draw top bar
    let top_text = Spans::from(vec![
        Span::styled("Controls:", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" 'h' - "),
        Span::styled("left,", Style::default().add_modifier(Modifier::ITALIC)),
        Span::raw(" 'j' - "),
        Span::styled("down,", Style::default().add_modifier(Modifier::ITALIC)),
        Span::raw(" 'k' - "),
        Span::styled("up,", Style::default().add_modifier(Modifier::ITALIC)),
        Span::raw(" 'l' - "),
        Span::styled("right", Style::default().add_modifier(Modifier::ITALIC)),
    ]);
    let top_text = Paragraph::new(top_text).alignment(Alignment::Center);
    f.render_widget(top_text, bar_area);

    let top_border = Block::default().borders(Borders::TOP);
    f.render_widget(top_border, border_area);

    let list_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        );
    let chunks = list_layout.split(main_area);
    let select_list_area = chunks[0];
    let demo_list_area = chunks[1];
    let code_area = chunks[2];

    //println!("{:?}", chunks);
    let selections = vec![
        ListItem::new("basic"), // 0
        ListItem::new("separated"),
        ListItem::new("fixed"),
    ];
    state.picker.resize(3);

    match state.picker.selected() {
        0 => demos::basic(demo_list_area, state, f),
        1 => demos::separated(demo_list_area, state, f),
        2 => demos::fixed(demo_list_area, state, f),
        _ => unreachable!(),
    }

    let bstyle = Style::default().fg(Color::Green);
    let select_bounds = Block::default()
        .borders(Borders::ALL)
        .title("demo")
        .border_type(BorderType::Thick)
        .style(bstyle);

    let select_list = SeparatedList::default()
        .block(select_bounds)
        .default_style(Style::reset().bg(Color::Black).fg(Color::White))
        .selected_style(Style::default().bg(Color::Blue).fg(Color::White))
        .items(selections)
        .item_display(ItemDisplay::Separated);
    f.render_stateful_widget(select_list, select_list_area, &mut state.picker);
}

mod demos {
    use super::*;

    pub(super) fn basic<B: Backend>(area: Rect, state: &mut AppState, f: &mut Frame<B>) {
        let demo_items = words();
        let demo_list = SeparatedList::default()
            .default_style(Style::reset().bg(Color::Black).fg(Color::White))
            .selected_style(Style::default().bg(Color::Blue).fg(Color::White))
            .items(demo_items)
            .item_display(ItemDisplay::Basic);
        f.render_stateful_widget(demo_list, area, &mut state.examples);
    }

    pub(super) fn separated<B: Backend>(area: Rect, state: &mut AppState, f: &mut Frame<B>) {
        let demo_items = words();
        let demo_list = SeparatedList::default()
            .default_style(Style::reset().bg(Color::Black).fg(Color::White))
            .selected_style(Style::default().bg(Color::Blue).fg(Color::White))
            .items(demo_items)
            .item_display(ItemDisplay::Separated);
        f.render_stateful_widget(demo_list, area, &mut state.examples);
    }

    pub(super) fn fixed<B: Backend>(area: Rect, state: &mut AppState, f: &mut Frame<B>) {
        let demo_items = words();
        let demo_list = SeparatedList::default()
            .default_style(Style::reset().bg(Color::Black).fg(Color::White))
            .selected_style(Style::default().bg(Color::Blue).fg(Color::White))
            .items(demo_items)
            .item_display(ItemDisplay::Basic)
            .window_type(WindowType::Fixed);

        f.render_stateful_widget(demo_list, area, &mut state.examples);
    }
}