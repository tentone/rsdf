use std::io;
use rand::Rng;
use rand::rngs::ThreadRng;

fn main() {
    println!("Guess the number!");
    println!("Input: ");

    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let mut rng: ThreadRng = rand::thread_rng();
    let secret: i32 = rng.gen_range(1..10);

    println!("You guessed {} it was {}", guess, secret);
}
