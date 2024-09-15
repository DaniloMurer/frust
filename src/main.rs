mod frust;
use std::io;
use std::io::Stdout;
use std::vec;

use frust::config::get_configs;
use frust::core::update_file_version;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Alignment, Constraint};
use ratatui::prelude::CrosstermBackend;
use ratatui::style::{Style, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, Row, Table, TableState};
use ratatui::Terminal;

struct App {
    table_state: TableState,
    title: &'static str,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl App {
    fn default() -> Self {
        Self {
            table_state: TableState::default(),
            title: " Frust ",
            terminal: ratatui::init(),
        }
    }

    fn run(&mut self) -> io::Result<()> {
        //let home_path = env::var("HOME").unwrap();
        let configs = get_configs("/home/churrer/Documents/github/frust/test");
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
                let block = Block::bordered()
                    .title(header_text.alignment(Alignment::Center))
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
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('j') {
                    self.table_state.scroll_down_by(1);
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('k') {
                    self.table_state.scroll_up_by(1);
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('l') {
                    let selected_row_index = self.table_state.selected().unwrap();
                    let selected_config = &configs[selected_row_index];
                    update_file_version(selected_config, &String::from("churrer.xyz:1.0.0"))
                        .expect("error while updating file version");
                }
            }
        }
    }

    fn clear(mut self) -> io::Result<()> {
        self.terminal.clear()
    }
}

fn main() -> io::Result<()> {
    let mut app = App::default();
    app.run()?;
    app.clear()
}
