use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read, Write};
use std::path::Path;
use std::{
    io,
    process::{Command, ExitStatus},
};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::Alignment;
use ratatui::symbols::border;
use ratatui::text::Text;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, Paragraph};
use ratatui::DefaultTerminal;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    location: Location
}

#[derive(Deserialize, Debug)]
struct Location {
    location: String,
    previous_version: String
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    run(terminal)
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    //let home_path = env::var("HOME").unwrap();
    let configs_path: String = "/home/churrer/Documents/github/frust/test/churrer.xyz.toml".to_owned();
    let config = read_config_toml(&configs_path);
    loop {
        terminal.draw(|frame| {
            let header_text = Title::from(" Frust ");
            let block = Block::bordered()
                .title(header_text.alignment(Alignment::Center))
                .border_set(border::THICK);
            let content = Paragraph::new(Text::from(config.name.clone()))
                .centered()
                .block(block);
            frame.render_widget(content, frame.area());
        })?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

fn read_config_toml(file_path: &String) -> Config
{
    let mut file = File::open(file_path).expect("error");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Error while reading file to string");
    let config: Config = toml::from_str(&buf).unwrap();
    config

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
