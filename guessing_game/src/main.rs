// STD
use std::io;
// Random lib
use rand::Rng;
// Compare lib?
use std::cmp::Ordering;

fn main() {
    /********/
    let mut secret = rand::thread_rng().gen_range(1..100);
    println!("The secret number is: {}. You can't see me again", secret);

    loop {
        let mut input = String::new();
        /********/
        println!("Enter a number: ");

        /********/
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Convert to u32
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bruh that's not a number between 1 and 100");
                continue;
            }
        };

        // Printf
        println!("You guessed: {}", input);

        // Match
        match input.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! - Restarting");
                // Reset secret
                secret = rand::thread_rng().gen_range(1..100);
            }
        }
    }
}
