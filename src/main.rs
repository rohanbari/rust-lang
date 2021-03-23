use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("=== Guess The Number Game ===");

    // Using Rng trait to generate unique random numbers
    let secret_number = rand::thread_rng().gen_range(1, 101);

    print!("Please input your guess. ");

    // Accepting user input as a string
    let mut guess = String::new();

    // Reading the input, some notables things:
    //   - The parameter  guess  is passed as a mutable-reference
    //   - The variables passed as arguments are considered immutable objects
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    print!("You guessed {} ", guess);

    // Chain method call, trimming the whitespaces, then parsing and handling
    // any possible error that might happen during parse operation
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // Comparing the  secret_number  with  guess  by using Ordering trait
    match guess.cmp(&secret_number) {
        Ordering::Greater => println!("Greater than expected."),
        Ordering::Less => println!("Lesser than expected."),
        Ordering::Equal => println!("CORRECT!")
    };

    println!("The number was {}", secret_number);
}
