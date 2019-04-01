extern crate fibonacci;
use::fibonacci::fib;
use std::io;
use std::io::Write;

fn main() {
    println!("Fibonacci Generator\n");
    println!("Calculate the n'th number of the Fibonacci sequence");
    println!("Type \"quit\" to end the program");

    loop {
        let mut n = String::new();

        // Print directly to stdout without a newline, flush stdout
        print!("\nEnter a positive integer from 1 to 184: ");
        io::stdout().flush()
            .unwrap();

        // Read a line of user input
        io::stdin().read_line(&mut n)
            .expect("Failed to read line");

        // If user input is quit, then break program loop
        if n.trim() == "quit" {
            break;
        }

        // Parse input into a number
        let n: u8 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if !fib::validate_input(n) {
            println!("Invalid number!");
        } else {
            let v = fib::fibonacci_to_nth(n);
            for i in &v {
                println!("{}", i);
            }

            let (a, b, c) = fib::nth_fibonacci(n);
            println!("The {}'th Fibonacci number is {} + {} = {}",
                     n, a, b, c);
        }
    }
}
