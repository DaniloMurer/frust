use std::{os::linux::raw::stat, process::{Command, ExitStatus}};

/// [`User`]
///
struct User {
    username: String,
    age: usize,
}

impl User {
    fn calculate_age(&self) -> usize {
        self.age.pow(2)
    }
}

fn main() {
    let user: User = User {
        username: String::from("churrer"),
        age: 16,
    };
    println!("User's age is: {}", calculate_age(&user));
    let output = Command::new("ls").output().expect("Failed to run command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", &output.status);
    let commit_status = commit_changes();
}

fn commit_changes() -> ExitStatus {
    let commit = Command::new("git").args(["commit", "-am", "feat: bumped version"]).status();
    commit.expect("")
}

fn calculate_age(user: &User) -> usize {
    println!("username: {}, address: {:p}", &user.username, &user);
    user.calculate_age()
}
