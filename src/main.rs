use rand::{thread_rng, Rng};
use std::io;

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
        if guess.cmp(&random_number).is_eq() {
            println!("You guess correctly. The number was: {}", guess);
            has_guessed = true;
        } else {
            println!("You guessed wrong");
        }
    }
}
