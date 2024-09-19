use std::io;
use std::io::Stdout;
use std::vec;

use super::config::get_configs;
use super::core::update_file_version;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
use ratatui::prelude::CrosstermBackend;
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, Clear, Paragraph, Row, Table, TableState};
use ratatui::Terminal;

enum InputMode {
    Editing,
    Normal,
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
            input_mode: InputMode::Normal,
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

    fn reset_input(&mut self) {
        self.input = String::new();
    }
}

pub struct App {
    table_state: TableState,
    title: &'static str,
    footer: &'static str,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    table_widths: [Constraint; 2],
    show_popup: bool,
}

impl App {
    pub fn default() -> Self {
        Self {
            table_state: TableState::default().with_selected(0),
            title: " Frust ",
            footer: " <q>: quit, <j>: navigate down, <k>: navigate up, <l>: run selected config ",
            terminal: ratatui::init(),
            table_widths: [Constraint::Length(50), Constraint::Length(100)],
            show_popup: false,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let configs = get_configs().unwrap_or(vec![]);
        let mut rows: Vec<Row> = vec![];

        for test in configs.iter() {
            rows.push(Row::new(vec![
                test.name.clone(),
                test.location.location.clone(),
            ]));
        }
        let mut input_field = InputField::new();
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
                let table = Table::new(rows.to_owned(), self.table_widths)
                    .column_spacing(1)
                    .header(
                        Row::new(vec!["Config Name", "Config Location"])
                            .style(Style::new().bold())
                            .bottom_margin(1),
                    )
                    .highlight_style(Style::new().reversed())
                    .highlight_symbol(">")
                    .block(block);

                frame.render_stateful_widget(table, frame.area(), &mut self.table_state);

                if self.show_popup {
                    let popup_area = create_popup_area(frame.area());
                    let input_widget = Paragraph::new(input_field.input.as_str())
                        .style(match input_field.input_mode {
                            InputMode::Normal => Style::default(),
                            InputMode::Editing => Style::default()
                                .fg(ratatui::style::Color::Magenta)
                                .add_modifier(Modifier::RAPID_BLINK),
                        })
                        .block(Block::bordered().title("Input"));
                    input_field.input_mode = InputMode::Editing;
                    frame.render_widget(Clear, popup_area);
                    frame.render_widget(input_widget, popup_area);
                    match input_field.input_mode {
                        InputMode::Normal => {}
                        InputMode::Editing => {
                            frame.set_cursor_position(ratatui::layout::Position::new(
                                popup_area.x + input_field.character_index as u16 + 1,
                                popup_area.y + 1,
                            ))
                        }
                    }
                }
            })?;
            if let Event::Key(key) = event::read()? {
                match input_field.input_mode {
                    InputMode::Normal => {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Char('q') => {
                                    if self.show_popup {
                                        self.show_popup = false;
                                    } else {
                                        return Ok(());
                                    }
                                }
                                KeyCode::Char('j') => self.table_state.scroll_down_by(1),
                                KeyCode::Char('k') => self.table_state.scroll_up_by(1),
                                KeyCode::Enter => {
                                    if self.show_popup {
                                        let selected_row_index =
                                            self.table_state.selected().unwrap();
                                        let selected_config = &configs[selected_row_index];
                                        update_file_version(
                                            selected_config,
                                            &String::from("churrer.xyz:1.0.0"),
                                        )
                                        .expect("error while updating file version");
                                        self.show_popup = false;
                                    } else {
                                        self.show_popup = true;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    InputMode::Editing => {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Enter => {
                                    input_field.reset_input();
                                    input_field.reset_cursor();
                                    input_field.input_mode = InputMode::Normal;
                                    self.show_popup = false;
                                }
                                KeyCode::Char(to_insert) => input_field.enter_char(to_insert),
                                KeyCode::Backspace => input_field.delete_char(),
                                KeyCode::Left => input_field.move_cursor_left(),
                                KeyCode::Right => input_field.move_cursor_right(),
                                KeyCode::Esc => {
                                    input_field.reset_input();
                                    input_field.reset_cursor();
                                    input_field.input_mode = InputMode::Normal;
                                    self.show_popup = false;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn clear(mut self) -> io::Result<()> {
        self.terminal.clear()
    }
}

fn create_popup_area(area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(20)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(80)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
