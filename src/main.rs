use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use std::{env, io, process::{Command, ExitStatus}};

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let location = args.get(1).expect("error please provide arguments [gitops_repo_location old_version new_version]");
    let old_version = args.get(2).expect("error please provide arguments [gitops_repo_location old_version new_version]");
    let new_version = args.get(3).expect("error please provide arguments [gitops_repo_location old_version new_version]");

    assert_eq!(location.is_empty(), false, "provide a location");
    assert_eq!(old_version.is_empty(), false, "provide a version to replace");
    assert_eq!(new_version.is_empty(), false, "provide a version to bump to");

    let lines = read_file_content(&location)?;
    for line in lines {
        if let Ok(line_content) = line {
            println!("{}", line_content);
        }
    }

    // commit_changes(location);
    dbg!(&args);
    Ok(())
    // if commit_changes(location).success() {
    //     // we good to push
    //     assert!(push_changes().success());
    // }
}

/// Reads the content from a file line by line.
///
/// # Arguments
///
/// * `file_path` - A path that references the file to be read.
///
/// # Returns
///
/// This function returns an `io::Result` containing an iterator over the lines of the file.
/// Each item of the iterator is a `Result` where `Ok` is a line in the file
/// and `Err` is an `io::Error`.
/// ```
fn read_file_content<P>(file_path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}


/// Commits changes in defined repo based on location
///
/// # Arguments
///
/// * `location` - Path to repository
///
/// # Returns
///
/// `ExitCode` - Exit code of the git commit command
///
fn commit_changes(location: &String) -> ExitStatus {
    let mut commit = Command::new("git");
    commit.args(["-C", location, "commit", "-am", "feat: bumped version"]);
    let commit_status = commit.status().expect("error while getting exit code");
    commit_status
}

/// Pushes commited changes upstream in defined repo based on location
///
/// # Arguments
///
/// * `location` - Path to repository
///
/// # Returns
///
/// `ExitCode` - Exit code of the git push command
///
fn push_changes(location: &String) -> ExitStatus {
    let mut push = Command::new("git");
    push.args(["-C", location, "push"]);
    return push.status().expect("error while getting exit code");
}

