use std::error::Error;
use std::io;
use std::io::Write;
extern crate fibonacci;
use fibonacci::fib;
extern crate num_format;
use num_format::{Locale, ToFormattedString};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Fibonacci Generator\n");
    println!("Calculate the n'th number of the Fibonacci sequence");
    println!("Type \"quit\" or \"exit\" to end the program");

    loop {
        let mut input = String::new();

        // Print directly to stdout without a newline, flush stdout
        print!("\nEnter a positive integer from 1 to 184: ");
        io::stdout().flush().unwrap();

        // Read a line of user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // If user input is quit, then break program loop
        if input.trim() == "quit" || input.trim() == "exit" {
            return Ok(println!("Goodbye!"));
        }

        // Parse input into a number
        let input: i64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };

        let v = match fib::fibonacci_to_nth(input) {
            Ok(vec) => vec,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        for (count, i) in v.iter().enumerate() {
            println!("{}: {}", count, i.to_formatted_string(&Locale::en));
        }

        let nth = fib::nth_fibonacci(input)?;
        match input {
            1 => println!("The 1'st Fibonacci number is {}", nth),
            2 => println!("The 2'nd Fibonacci number is {}", nth),
            3 => println!("The 3'rd Fibonacci number is {}", nth),
            _ => println!(
                "The {}'th Fibonacci number is {}",
                input,
                nth.to_formatted_string(&Locale::en)
            ),
        }
    }
}
