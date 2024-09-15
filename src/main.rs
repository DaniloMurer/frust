use core::panic;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Lines, Read, Write};
use std::path::Path;
use std::vec;
use std::{
    io,
    process::{Command, ExitStatus},
};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Alignment, Constraint};
use ratatui::style::{Style, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, Row, Table, TableState};
use ratatui::DefaultTerminal;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    location: Location,
}

#[derive(Deserialize, Debug)]
struct Location {
    location: String,
    previous_version: String,
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    run(terminal)
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
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
    let mut table_state = TableState::default();
    loop {
        terminal.draw(|frame| {
            let header_text = Title::from(" Frust ");
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
            frame.render_stateful_widget(table, frame.area(), &mut table_state);
        })?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('j') {
                table_state.scroll_down_by(1);
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('k') {
                table_state.scroll_up_by(1);
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('l') {
                let selected_row_index = table_state.selected().unwrap();
                let selected_config = &configs[selected_row_index];
                update_file_version(selected_config, &String::from("churrer.xyz:1.0.0"));
            }
        }
    }
}

fn get_configs(config_directory: &'static str) -> Vec<Config> {
    let toml_paths = fs::read_dir(config_directory).unwrap();
    let mut return_paths: Vec<String> = vec![];
    let mut configs: Vec<Config> = vec![];

    for toml_path in toml_paths {
        let raw_path = toml_path.unwrap().path().to_str().unwrap().to_string();
        if raw_path.contains(".toml") {
            return_paths.push(raw_path);
        }
    }
    for path in return_paths {
        configs.push(read_config_toml(path));
    }
    configs
}

fn read_config_toml(file_path: String) -> Config {
    let mut file = File::open(file_path).expect("error");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Error while reading file to string");
    let config: Config = toml::from_str(&buf).unwrap();
    config
}

fn update_file_version(config: &Config, new_version: &String) {
    let lines = read_file_content(&config.location.location);
    let lines = match lines {
        Ok(buffer) => buffer,
        Err(_) => panic!("Erro while reading file content, make sure path is valid"),
    };
    let mut new_lines: Vec<String> = vec![];
    for line in lines {
        let mut line_content = line.unwrap();
        if line_content.contains(&config.location.previous_version) {
            line_content = line_content.replace(&config.location.previous_version, new_version);
        }
        new_lines.push(line_content);
    }

    let result = write_to_file(&config.location.location, new_lines);
    match result {
        Ok(_) => (),
        Err(_) => eprint!("Error"),
    }
}

/// Reads the content from a file line by line.
///
/// # Arguments
///
/// * `file_path` - A path that references the file to be read.
///
/// # Returns
///
/// This function returns an [`io::Result`] containing an iterator over the lines of the file.
/// Each item of the iterator is a [`Result`] where [`Ok`] is a line in the file
/// and [`Err`] is an [`io::Error`].
/// ```
fn read_file_content<P>(file_path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}

/// Writes content to file
///
/// # Arguments
/// * `location` - Path to write to.
/// * `file_content` - File content to write as a [`Vec<String>`] vector.
///
/// # Returns
///
/// Function returns a [`io::Result`].
/// If an error occurs while writing to file, a [`Err`] is returned.
/// Upon successful writing to file [`Ok`] is returned
fn write_to_file(file_path: &String, file_content: Vec<String>) -> io::Result<()> {
    let file = File::create(file_path);
    file?.write_all((file_content.join("\n")).as_ref())
}

/// Commits changes in defined repo based on `location`
///
/// # Arguments
///
/// * `location` - Path to changed file
///
/// # Returns
///
/// [`ExitCode`] - Exit code of the git commit command
///
fn commit_changes(location: &String, new_version: &String) -> ExitStatus {
    let mut commit = Command::new("git");
    let file_path = Path::new(location);
    let parent_path = file_path.parent().unwrap();
    commit.args([
        "-C",
        parent_path.to_str().unwrap(),
        "commit",
        "-am",
        &format!("feat: bumped to version {}", new_version),
    ]);
    commit.status().expect("error while getting exit code")
}

/// Pushes commited changes upstream in defined repo based on `location`
///
/// # Arguments
///
/// * `location` - Path to changed file
///
/// # Returns
///
/// [`ExitCode`] - Exit code of the git push command
///
fn push_changes(location: &String) -> ExitStatus {
    let mut push = Command::new("git");
    let file_path = Path::new(location);
    let parent_path = file_path.parent().unwrap();
    push.args(["-C", parent_path.to_str().unwrap(), "push"]);
    push.status().expect("error while getting exit code")
}
