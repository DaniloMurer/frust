use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines, Write};
use std::path::Path;
use std::{
    io,
    process::{Command, ExitStatus},
};

use super::config::Config;

pub fn update_file_version(config: &Config, new_version: &String) -> Result<(), Error> {
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
        Ok(_) => Ok(()),
        Err(error) => Err(error),
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
