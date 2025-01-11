use clap::Parser;
use fibonacci::fib::{fibonacci_to_nth, nth_fibonacci};
use num_format::{Locale, ToFormattedString};
use std::{error::Error, io, io::Write};

#[derive(Parser)]
#[command(arg_required_else_help(true))]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    /// Fibonacci numbers to N
    #[arg(short = 'N', long = "to-nth", value_name = "NUMBER")]
    to_nth: Option<usize>,

    /// N'th Fibonacci number
    #[arg(short, long, value_name = "NUMBER")]
    nth: Option<usize>,

    /// Interactive Fibonacci generator repl
    #[arg(short, long)]
    repl: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line
    let cli = Cli::parse();

    // Print Fibonacci numbers to n'th element
    if let Some(to_nth) = cli.to_nth.as_ref() {
        run_fibonacci_to_nth(*to_nth)?;
        return Ok(());
    }

    // Print n'th Fibonacci number
    if let Some(nth) = cli.nth.as_ref() {
        run_nth_fibonacci(*nth)?;
        return Ok(());
    }

    // Interactive repl
    if cli.repl {
        interactive()?
    }
    Ok(())
}

/// Print Fibonacci numbers up to the n'th element
fn run_fibonacci_to_nth(n: usize) -> Result<(), &'static str> {
    // Create a vector of Fibonacci numbers up to the n'th element
    let vec = match fibonacci_to_nth(n) {
        Some(v) => v,
        None => return Err("fib::fibonacci_to_nth failed"),
    };

    // Iterate the vector and print an enumerated list of numbers
    for (idx, n) in vec.iter().enumerate() {
        println!("{}: {}", idx, n.to_formatted_string(&Locale::en));
    }
    Ok(())
}

/// Print the n'th Fibonacci number
fn run_nth_fibonacci(nth: usize) -> Result<(), &'static str> {
    // Calculate the N'th Fibonacci number
    if let Some(n) = nth_fibonacci(nth) {
        println!(
            "Number {} in the Fibonacci series is {}",
            nth,
            n.to_formatted_string(&Locale::en)
        );
        Ok(())
    } else {
        Err("fib::nth_fibonacci failed")
    }
}

/// REPL to calculate Fibonacci numbers
fn interactive() -> Result<(), Box<dyn Error>> {
    println!("Fibonacci Generator\n");
    println!("Calculate the n'th number of the Fibonacci sequence");
    println!("Type \"quit\" or \"exit\" to end the program");

    loop {
        let mut input = String::new();

        // Print directly to stdout without a newline, flush stdout
        print!("\nEnter a number up to 185: ");
        io::stdout().flush().unwrap();

        // Read a line of user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // If user input is quit, then break program loop
        if input.trim() == "quit" || input.trim() == "exit" {
            println!("Goodbye!");
            return Ok(());
        }

        // Parse input into a number
        let nth: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid input!");
                continue;
            }
        };

        // Run the Fibonnaci functions
        if let Err(_) = run_fibonacci_to_nth(nth) {
            continue;
        };
        println!();
        if let Err(_) = run_nth_fibonacci(nth) {
            continue;
        };
    }
}
