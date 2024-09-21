use std::io;
use std::io::Stdout;
use std::rc::Rc;
use std::vec;

use super::components::input_field;
use super::components::input_field::InputField;
use super::components::input_field::InputMode;
use super::config::get_configs;
use super::core::update_file_version;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::Direction;
use ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
use ratatui::prelude::CrosstermBackend;
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, Clear, Paragraph, Row, Table, TableState};
use ratatui::Terminal;

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
        let configs = get_configs().unwrap_or_default();
        let mut rows: Vec<Row> = vec![];

        for test in configs.iter() {
            rows.push(Row::new(vec![
                test.name.clone(),
                test.location.location.clone(),
            ]));
        }
        let mut input_field_old_version = InputField::new();
        let mut input_field_new_version = InputField::new();
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
                    let popup_layout = create_input_layout(popup_area);
                    let input_old_version = Paragraph::new(input_field_old_version.input.as_str())
                        .style(match input_field_old_version.input_mode {
                            InputMode::Normal => Style::default(),
                            InputMode::Editing => Style::default()
                                .fg(ratatui::style::Color::Magenta)
                                .add_modifier(Modifier::RAPID_BLINK),
                        })
                        .block(Block::bordered().title("Old version"));
                    input_field_old_version.input_mode = InputMode::Normal;
                    let input_new_version = Paragraph::new(input_field_new_version.input.as_str())
                        .style(match input_field_new_version.input_mode {
                            InputMode::Normal => Style::default(),
                            InputMode::Editing => Style::default()
                                .fg(ratatui::style::Color::Magenta)
                                .add_modifier(Modifier::RAPID_BLINK),
                        })
                        .block(Block::bordered().title("New version"));
                    input_field_new_version.input_mode = InputMode::Editing;
                    frame.render_widget(Clear, popup_area);
                    frame.render_widget(input_old_version, popup_layout[0]);
                    frame.render_widget(input_new_version, popup_layout[1]);
                    match input_field_old_version.input_mode {
                        InputMode::Normal => {}
                        InputMode::Editing => {
                            frame.set_cursor_position(ratatui::layout::Position::new(
                                popup_area.x + input_field_old_version.character_index as u16 + 1,
                                popup_area.y + 1,
                            ))
                        }
                    }
                }
            })?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Tab {
                    if input_field_old_version.input_mode == InputMode::Normal {
                        input_field_old_version.input_mode = InputMode::Editing;
                        input_field_new_version.input_mode = InputMode::Normal;
                    } else {
                        input_field_old_version.input_mode = InputMode::Normal;
                        input_field_new_version.input_mode = InputMode::Editing;
                    }
                }
                match input_field_old_version.input_mode {
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
                                    input_field_old_version.reset_input();
                                    input_field_old_version.reset_cursor();
                                    input_field_old_version.input_mode = InputMode::Normal;
                                    self.show_popup = false;
                                }
                                KeyCode::Char(to_insert) => {
                                    input_field_old_version.enter_char(to_insert)
                                }
                                KeyCode::Backspace => input_field_old_version.delete_char(),
                                KeyCode::Left => input_field_old_version.move_cursor_left(),
                                KeyCode::Right => input_field_old_version.move_cursor_right(),
                                KeyCode::Esc => {
                                    input_field_old_version.reset_input();
                                    input_field_old_version.reset_cursor();
                                    input_field_old_version.input_mode = InputMode::Normal;
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

fn create_input_layout(area: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area)
}
