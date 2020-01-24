use std::{convert::TryInto, env, error::Error, io, io::Write, process};

use fibonacci::{
    fib::{fibonacci_to_nth, nth_fibonacci},
    parse_command_line,
};

use getargs::Options;
use num_format::{Locale, ToFormattedString};

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args: Vec<_> = env::args().skip(1).collect();
    let opts = Options::new(&args);
    let options = match parse_command_line(&opts) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("usage error: {}", e);
            process::exit(1);
        }
    };

    // Print help message and exit
    if options.help {
        help();
        process::exit(0);
    }

    // Run one of the two Fibonacci commands, an interactive program,
    // or display a help message
    if options.nth > 0 {
        let n: isize = options.nth.try_into().unwrap();
        run_nth_fibonacci(n);
    } else if options.to_nth > 0 {
        let n: isize = options.to_nth.try_into().unwrap();
        run_fibonacci_to_nth(n);
    } else if options.interactive {
        interactive()?;
    } else {
        help();
    }

    Ok(())
}

fn help() {
    println!("Interactive Fibonacci Program Help\n");
    println!("Commands can be written -e EXPRESSION, or -eEXPRESSION, or");
    println!("--execute EXPRESSION, or --execute=EXPRESSION");
    println!("Valid Commands:\n");
    println!("-n num or --nth num");
    println!("\tprint the n'th Fibonacci number\n");
    println!("-N num or --to_nth num");
    println!("\tprint a list of Fibonacci numbers up to the n'th element\n");
    println!("-i or --interactive");
    println!("\trun an interactive version of the program\n");
    println!("-h or --help");
    println!("\tprint this help message \u{1F600}\n");
}

fn run_nth_fibonacci(n: isize) {
    // Determine the maxmimum value for vector below
    let limit = match nth_fibonacci(n) {
        Ok(n) => n,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };

    // List of of 3 + 10..limit
    let mut three_plus_tens: Vec<u128> = vec![3];
    for i in (3..limit).step_by(10) {
        three_plus_tens.push(i);
    }

    let _ = match nth_fibonacci(n) {
        Ok(nth) => match n {
            // Try for proper grammar, 1'st, 2'nd, 3'rd etc
            1 => println!("The 1'st Fibonacci number is {}", nth),
            2 => println!("The 2'nd Fibonacci number is {}", nth),
            _ => {
                for &i in three_plus_tens.iter() {
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
        },
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };
}

fn run_fibonacci_to_nth(n: isize) {
    // Create a vector of Fibonacci numbers up to the n'th element
    let vec = match fibonacci_to_nth(n) {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}", e);
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
