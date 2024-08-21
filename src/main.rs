use std::{env, process::{Command, ExitStatus}};

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let location = args.get(1).expect("error please provide arguments [gitops_repo_location old_version new_version]");
    let old_version = args.get(2).expect("error please provide arguments [gitops_repo_location old_version new_version]");
    let new_version = args.get(3).expect("error please provide arguments [gitops_repo_location old_version new_version]");

    assert_eq!(location.is_empty(), false, "provide a location");
    assert_eq!(old_version.is_empty(), false, "provide a version to replace");
    assert_eq!(new_version.is_empty(), false, "provide a version to bump to");

    dbg!(args);
    // if commit_changes(location).success() {
    //     // we good to push
    //     assert!(push_changes().success());
    // }
}

/// commit_changes stages and commits current changes
fn commit_changes(location: &String) -> ExitStatus {
    let mut commit = Command::new("git");
    commit.args(["-C", location, "commit", "-am", "feat: bumped version"]);
    let commit_status = commit.status().expect("error while getting exit code");
    commit_status
}

/// push_changes pushes staged & commited changes
fn push_changes(location: &String) -> ExitStatus {
    let mut push = Command::new("git");
    push.args(["-C", location, "push"]);
    return push.status().expect("error while getting exit code");
}

