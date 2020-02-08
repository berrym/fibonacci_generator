# fibonacci_generator

Fibonacci number generator.

## Description

Small command line utility to generate the Fibonacci series of numbers.

## Getting started

Install a recent version of Rust using your OS distributions package manager or Mozilla's own preferred rustup.  For details check with your OS distribution or visit https://rust-lang.org for more information.

### Installing

Clone the git repository from https://github.com/berrym/fibonacci_generator.git

### Executing program

Use Rust's own tooling to compile and run the program, e.g.

* cargo run

## Help

The available commands can be run by specifying command line switches.
These switches are described in the help which can be run by issuing the command

* cargo run
    * cargo run -- -h
	* cargo run -- --help

* cargo run -- -n 23
    * cargo run -- --nth 23

* cargo run -- --N 13
    * cargo run -- --to_nth 13

* cargo run -- -i
    * cargo run -- --interactive

## Authors

Copyright 2020
Michael Berry <trismegustis@gmail.com>

## Version History
* 0.1.7
    * Grammar rules based on vector analysis removed.
* 0.1.6
    * Use clap to parse command line
* 0.1.5
    * Improved grammar when printing singular results
* 0.1.4
    * Use getargs to process command line
	* Split main binary into multiple functions
	* Added this README.md and LICENSE file
* 0.1.3
    * Refactoring
* 0.1.2
    * Refactoring
* 0.1.0
    * Initial Release

## License

This project is licensed under the MIT License - see the LICENSE file  for details.

## Acknowledgments

The excellent and freely available Rust book, for more information visit https://rust-lang.org
