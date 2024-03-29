use clap::{App, Arg};
use fibonacci::fib::{fibonacci_to_nth, nth_fibonacci};
use num_format::{Locale, ToFormattedString};
use std::{error::Error, io, io::Write};

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line
    let cli = App::new("Fibonacci Generator")
        .version("0.1.8")
        .author("Michael Berry <trismegustis@gmail.com>")
        .about("Generate the Fibonacci sequence")
        .arg(
            Arg::with_name("Fibonacci to N'th")
                .short("f")
                .long("fibonacci-to")
                .help("Generate Fibonacci sequence up to N'th number")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("N'th Fibonacci")
                .short("F")
                .long("nth-fibonacci")
                .help("Calculate the N'th Fibonacci number")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("interactive")
                .short("i")
                .long("interactive")
                .help("Interactive program"),
        )
        .get_matches();

    if cli.is_present("Fibonacci to N'th") {
        if let Some(n) = cli.value_of("Fibonacci to N'th") {
            match n.parse() {
                Ok(n) => run_fibonacci_to_nth(n)?,
                Err(e) => eprintln!("Error: {}", e)
            }
        };
    } else if cli.is_present("N'th Fibonacci") {
        if let Some(n) = cli.value_of("N'th Fibonacci") {
            match n.parse() {
                Ok(n) => run_nth_fibonacci(n)?,
                Err(e) => eprintln!("Error: {}", e)
            }
        };
    } else if cli.is_present("interactive") {
        interactive()?;
    } else {
        eprintln!("{}\n\nTry passing --help for more information", cli.usage());
    }
    Ok(())
}

fn run_fibonacci_to_nth(n: usize) -> Result<(), &'static str> {
    // Create a vector of Fibonacci numbers up to the n'th element
    let vec = match fibonacci_to_nth(n) {
        Some(v) => v,
        None => return Err("fib::fibonacci_to_nth failed")
    };

    // Iterate the vector and print an enumerated list of numbers
    for (index, n) in vec.iter().enumerate() {
        println!("{}: {}", index, n.to_formatted_string(&Locale::en));
    }
    Ok(())
}

fn run_nth_fibonacci(nth: usize) -> Result<(), &'static str> {
    // Calculate the N'th Fibonacci number
    if let Some(n) = nth_fibonacci(nth) {
        println!(
            "Element {} in the Fibonacci series is {}",
            nth,
            n.to_formatted_string(&Locale::en)
        );
        Ok(())
    } else {
        Err("fib::nth_fibonacci failed")
    }
}

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
        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid input!");
                continue;
            }
        };

        // Run the Fibonnaci functions
        if let Err(_) = run_fibonacci_to_nth(input) {
            continue;
        };
        if let Err(_) = run_nth_fibonacci(input) {
            continue;
        };
    }
}
