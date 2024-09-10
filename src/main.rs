use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Write};
use std::path::Path;
use std::{
    env, io,
    process::{Command, ExitStatus},
};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::style::Stylize;
use ratatui::widgets::Paragraph;
use ratatui::DefaultTerminal;

/// Hold arguments for cli
struct Arguments {
    location: String,
    old_version: String,
    new_version: String,
    commit_flag: bool,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 4 {
            return Err("error please provide required arguments [gitops_repo_location old_version new_version]");
        }

        let location = args[1].clone();
        let old_version = args[2].clone();
        let new_version = args[3].clone();
        let mut commit_flag = true;
        if args[4].clone() == "ng" {
            commit_flag = false;
        }

        Ok(Arguments {
            location,
            old_version,
            new_version,
            commit_flag,
        })
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    /*let args: Vec<String> = env::args().collect();

    let cli_args = Arguments::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    let lines = read_file_content(&cli_args.location)?;
    let mut new_lines: Vec<String> = Vec::new();
    for line in lines {
        if let Ok(mut line_content) = line {
            if line_content.contains(&cli_args.old_version) {
                line_content = line_content.replace(&cli_args.old_version, &cli_args.new_version);
            }
            new_lines.push(line_content);
        }
    }

    write_to_file(&cli_args.location, new_lines).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    if cli_args.commit_flag && commit_changes(&cli_args.location, &cli_args.new_version).success() {
        // we good to push
        assert!(push_changes(&cli_args.location).success());
    }
    dbg!(&args);*/
    run(terminal)
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let header_text = Paragraph::new("Welcome to Frust!")
                .white()
                .on_black()
                .centered();
            frame.render_widget(header_text, frame.area());
        })?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
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
