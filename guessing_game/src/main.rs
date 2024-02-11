use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess what number I'm thinking!");
    println!("Enter your guess.");

    // thread_rng defines the random number generator to use
    // gen_range tells the generator to generate a random number
    // within some given range. the = operator in the range defines inclusivity of 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // variables are immutable by default,
        // so explicitly make guess mutable as the guess is dynamic and unknown at build time
        let mut guess = String::new();
        // create a handle to the users standard input from terminal
        io::stdin()
            // references (indicated by &) are immutable
            // so pass mut to allow read_line to modify the variables' value
            .read_line(&mut guess)
            // because read_line returns a "Result" with two possible "states" or "variants",
            // we can call the "expect" method on it
            // which will allow us to prepend a custom message to any error message provided
            // by the "Err" variant.
            // the expect method will cause the program to crash upon an error.
            .expect("Failed to read line");

        // "shadowing" the guess variable by trimming out whitespaces
        // and parsing (implicitly) from a string to a u32.
        // using a match expression to define valid variants of the user input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // return the value of "num" and bind to "guess"
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // create a match expression that defines 3 "arms",
        // where each arm contains a pattern describing
        // what constitutes two numbers being "Less", "Greater", or "Equal"
        // to one another, and what action to trigger when that specific arm's pattern is matched
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
