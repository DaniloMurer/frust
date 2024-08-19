// use rand::{thread_rng, Rng};
// use std::{cmp::Ordering, io};

/// [`User`]
///
struct User {
    username: String,
    age: usize
}

impl User {
    fn calculate_age(&self) -> usize {
        self.age.pow(2)
    }
}

fn main() {
    let user: User = User{username: String::from("churrer"), age: 16 };
    println!("User's age is: {}", calculate_age(&user));
    // println!("guess the number");
    // let random_number: u8 = thread_rng().gen_range(1..=20);
    // let mut has_guessed = false;
    // while !has_guessed {
    //     let mut guess = String::new();
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line?");
    //     let guess: u8 = guess.trim().parse().expect("Error while casting");
    //
    //     match guess.cmp(&random_number) {
    //         Ordering::Less => println!("the number is bigger"),
    //         Ordering::Greater => println!("the number is smaller"),
    //         Ordering::Equal => {
    //             println!("you guess right: {}", guess);
    //             has_guessed = true;
    //         }
    //     }
    // }
}

fn calculate_age(user: &User) -> usize {
    println!("username: {}, address: {:p}", &user.username, &user);
    user.calculate_age()
}
