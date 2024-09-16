use std::io;
use std::io::Stdout;
use std::vec;

use super::config::get_configs;
use super::core::update_file_version;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Alignment, Constraint};
use ratatui::prelude::CrosstermBackend;
use ratatui::style::{Style, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, Row, Table, TableState};
use ratatui::Terminal;

#[allow(clippy::upper_case_acronyms)]
enum Key {
    QUIT,
    UP,
    DOWN,
    ENTER,
}

impl Key {
    fn value(self) -> KeyCode {
        match self {
            Self::QUIT => KeyCode::Char('q'),
            Self::UP => KeyCode::Char('k'),
            Self::DOWN => KeyCode::Char('j'),
            Self::ENTER => KeyCode::Char('l'),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
enum InputMode {
    EDITING,
    NORMAL,
}

struct InputField {
    input: String,
    character_index: usize,
    input_mode: InputMode,
}

impl InputField {
    const fn new() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::NORMAL,
            character_index: 0,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_left = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn clamp_cursor(&self, new_cursor_position: usize) -> usize {
        new_cursor_position.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }
}

pub struct App {
    table_state: TableState,
    title: &'static str,
    footer: &'static str,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl App {
    pub fn default() -> Self {
        Self {
            table_state: TableState::default(),
            title: " Frust ",
            footer: " <q>: quit, <j>: navigate down, <k>: navigate up, <l>: run selected config ",
            terminal: ratatui::init(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let configs = get_configs();
        let mut rows: Vec<Row> = vec![];
        let widths = [Constraint::Length(50), Constraint::Length(100)];

        for test in configs.iter() {
            rows.push(Row::new(vec![
                test.name.clone(),
                test.location.location.clone(),
            ]));
        }
        loop {
            self.terminal.draw(|frame| {
                let header_text = Title::from(self.title);
                let footer_text = Title::from(self.footer);
                let block = Block::bordered()
                    .title(header_text.alignment(Alignment::Center))
                    .title(
                        footer_text
                            .alignment(Alignment::Center)
                            .position(Position::Bottom),
                    )
                    .border_set(border::THICK);
                let table = Table::new(rows.to_owned(), widths)
                    .column_spacing(1)
                    .header(
                        Row::new(vec!["Config Name", "Config Location"])
                            .style(Style::new().bold())
                            .bottom_margin(1),
                    )
                    .highlight_style(Style::new().reversed())
                    .block(block);
                frame.render_stateful_widget(table, frame.area(), &mut self.table_state);
            })?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == Key::QUIT.value() {
                    return Ok(());
                }
                if key.kind == KeyEventKind::Press && key.code == Key::DOWN.value() {
                    self.table_state.scroll_down_by(1);
                }
                if key.kind == KeyEventKind::Press && key.code == Key::UP.value() {
                    self.table_state.scroll_up_by(1);
                }
                if key.kind == KeyEventKind::Press && key.code == Key::ENTER.value() {
                    let selected_row_index = self.table_state.selected().unwrap();
                    let selected_config = &configs[selected_row_index];
                    update_file_version(selected_config, &String::from("churrer.xyz:1.0.0"))
                        .expect("error while updating file version");
                }
            }
        }
    }

    pub fn clear(mut self) -> io::Result<()> {
        self.terminal.clear()
    }
}
