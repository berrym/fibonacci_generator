use std::{convert::TryInto, error::Error, io, io::Write, process};

use fibonacci::fib::{fibonacci_to_nth, nth_fibonacci};

use clap::{App, Arg};

use num_format::{Locale, ToFormattedString};

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line
    let cli = App::new("Fibonacci Generator")
        .version("0.1.6")
        .author("Michael Berry <trismegustis@gmail.com>")
        .about("Generate the Fibonacci series")
        .arg(
            Arg::with_name("fibonacci_to")
                .short("f")
                .long("fibonacci-to")
                .help("Generate Fibonacci series up to N")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("nth_fibonacci")
                .short("F")
                .long("nth-fibonacci")
                .help("Generate Fibonacci series up to N")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("interactive")
                .short("i")
                .long("interactive")
                .help("Interactive program"),
        )
        .get_matches();

    if cli.is_present("fibonacci_to") {
        if let Some(n) = cli.value_of("fibonacci_to") {
            match n.parse() {
                Ok(n) => run_fibonacci_to_nth(n),
                Err(e) => println!("Error: {}", e),
            }
            return Ok(());
        };
    } else if cli.is_present("nth_fibonacci") {
        if let Some(n) = cli.value_of("nth_fibonacci") {
            match n.parse() {
                Ok(n) => run_nth_fibonacci(n),
                Err(e) => println!("Error: {}", e),
            }
            return Ok(());
        };
    } else if cli.is_present("interactive") {
        interactive()?;
    } else {
        println!("{}\n\nTry passing --help for more information", cli.usage());
    }
    Ok(())
}

fn run_nth_fibonacci(n: isize) {
    // Determine the maxmimum value for the vectors below
    let limit = match nth_fibonacci(n) {
        Some(n) => n,
        None => {
            println!("Error occured");
            process::exit(1);
        }
    };

    // Vectors of numbers ending in 1, 2 and 3
    let ones = (1..=limit).step_by(10).collect::<Vec<_>>();
    let twos = (1..=limit).skip(1).step_by(10).collect::<Vec<_>>();
    let threes = (1..=limit).skip(2).step_by(10).collect::<Vec<_>>();

    match nth_fibonacci(n) {
        Some(nth) => {
            // Try for proper grammar, 1'st, 2'nd, 3'rd etc
            // Numbers ending in one
            if n == 11 {
                // Eleven is a special case for ones
                println!(
                    "The {}'th Fibonacci number is {}",
                    n,
                    nth.to_formatted_string(&Locale::en)
                );
                return;
            }
            for &i in ones.iter() {
                if i == n.try_into().unwrap() {
                    println!(
                        "The {}'st Fibonacci number is {}",
                        n,
                        nth.to_formatted_string(&Locale::en)
                    );
                    return;
                }
            }
            // Numbers ending in two
            for &i in twos.iter() {
                if i == n.try_into().unwrap() {
                    println!(
                        "The {}'nd Fibonacci number is {}",
                        n,
                        nth.to_formatted_string(&Locale::en)
                    );
                    return;
                }
            }
            // Numbers ending in three
            for &i in threes.iter() {
                if i == n.try_into().unwrap() {
                    println!(
                        "The {}'rd Fibonacci number is {}",
                        n,
                        nth.to_formatted_string(&Locale::en)
                    );
                    return;
                }
            }
            // Everything else
            println!(
                "The {}'th Fibonacci number is {}",
                n,
                nth.to_formatted_string(&Locale::en)
            );
        }
        None => {
            println!("Error occured");
            process::exit(1);
        }
    }
}

fn run_fibonacci_to_nth(n: isize) {
    // Create a vector of Fibonacci numbers up to the n'th element
    let vec = match fibonacci_to_nth(n) {
        Some(v) => v,
        None => {
            println!("Error occured");
            process::exit(1);
        }
    };

    // Iterate the vector and print an enumerated list of numbers
    for (index, n) in vec.iter().enumerate() {
        println!("{}: {}", index, n.to_formatted_string(&Locale::en));
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
            return Ok(println!("Goodbye!"));
        }

        // Parse input into a number
        let input: isize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };

        // Run the Fibonacci commands
        run_fibonacci_to_nth(input);
        run_nth_fibonacci(input);
    }
}
