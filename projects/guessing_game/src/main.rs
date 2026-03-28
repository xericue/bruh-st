use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number twin...");

    // immutable variable from rand library
    // using the thread_rng() function with the gen_range() method
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {}... Shhh ...", secret_number);
    println!("Enter your guess:");

    let mut guess = String::new();
    // hi!
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("dawg type a number...");

    println!("You guessed: {guess}");

    // Ordering is another enum (type with multiple variants) and thus
    // we define its outcome with each

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small :("),
        Ordering::Greater => println!("TOO BIG!"),
        Ordering::Equal => println!("PEAK!")
    }
}
