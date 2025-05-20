use std::io;

use crate::frontend::parser::Subtitle;
use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    prelude::Backend,
    style::{Color, Style, Stylize},
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListState, Scrollbar,
        ScrollbarOrientation, ScrollbarState,
    },
};

pub fn run_tui<B: Backend>(
    terminal: &mut Terminal<B>,
    subs: (&Vec<Subtitle>, usize),
    state: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui_draw(f, subs, state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => state.add(),
                KeyCode::Up => state.sub(),
                _ => {}
            }
        }
    }
}

fn ui_draw(frame: &mut Frame, subs: (&Vec<Subtitle>, usize), state: &mut App) {
    let page_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - 60) / 2),
            Constraint::Percentage(60),
            Constraint::Percentage((100 - 60) / 2),
        ])
        .split(frame.area());

    let app_block = Block::default()
        .title(format!("Line: {}/{}", state.scroll_pos + 1, state.max).yellow())
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue))
        .border_type(BorderType::Double);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    let list = List::default()
        .items(flatten(subs.0.clone()))
        .block(app_block)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_style(Style::default().bg(Color::White).fg(Color::Black));

    frame.render_stateful_widget(list, page_layout[1], &mut state.list_state);
    frame.render_stateful_widget(
        scrollbar,
        page_layout[1].inner(Margin {
            horizontal: 1,
            vertical: 0,
        }),
        &mut state.scroll_state,
    );
}

fn humanize(secs: u64) -> String {
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn flatten(nested: Vec<Subtitle>) -> Vec<String> {
    nested
        .into_iter()
        .flat_map(|s| {
            let time = s.start.unwrap();
            s.text
                .into_iter()
                .flatten()
                .map(move |line| format!("{}  {}", humanize(time.as_secs()), line))
        })
        .collect()
}

pub struct App {
    pub scroll_state: ScrollbarState,
    pub scroll_pos: usize,
    pub list_state: ListState,
    pub max: usize,
}

impl App {
    pub fn new(lines: usize) -> Self {
        Self {
            scroll_state: ScrollbarState::new(lines).position(1),
            scroll_pos: 0,
            list_state: ListState::default().with_selected(Some(0)),
            max: lines,
        }
    }

    pub fn add(&mut self) {
        if self.scroll_pos <= self.max - 2 {
            self.scroll_pos = self.scroll_pos.saturating_add(1);
            self.scroll_state = self.scroll_state.position(self.scroll_pos);

            self.list_state.select(Some(self.scroll_pos));
        }
    }

    pub fn sub(&mut self) {
        if self.scroll_pos > 0 {
            self.scroll_pos = self.scroll_pos.saturating_sub(1);
            self.scroll_state = self.scroll_state.position(self.scroll_pos);

            self.list_state.select(Some(self.scroll_pos));
        }
    }
}
