use rand::{thread_rng, Rng};
use std::{cmp::Ordering, io};

fn main() {
    println!("guess the number");
    let random_number = thread_rng().gen_range(1..=20);
    let mut has_guessed = false;
    while !has_guessed {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line?");
        let guess: i32 = guess.trim().parse().expect("Error while casting");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("the number is bigger"),
            Ordering::Greater => println!("the number is smaller"),
            Ordering::Equal => {
                println!("you guess right: {}", guess);
                has_guessed = true;
            }
        }
    }
}
