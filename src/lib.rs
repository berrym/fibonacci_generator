use getargs::{self, Opt, Options};

#[derive(Default, Debug)]
pub struct CommandLineArguments<'a> {
    pub nth: u32,
    pub to_nth: u32,
    pub interactive: bool,
    pub help: bool,
    positional_args: &'a [String],
}

pub fn parse_command_line<'a>(
    opts: &'a Options<'a, String>,
) -> getargs::Result<CommandLineArguments<'a>> {
    let mut result = CommandLineArguments::default();
    while let Some(opt) = opts.next() {
        match opt? {
            // -e EXPRESSION, or -eEXPRESSION, or
            // --execute EXPRESSION, or --execute=EXPRESSION
            Opt::Short('n') | Opt::Long("nth") => result.nth = opts.value()?,
            Opt::Short('N') | Opt::Long("to_nth") => result.to_nth = opts.value()?,
            Opt::Short('i') | Opt::Long("interactive") => result.interactive = true,
            Opt::Short('h') | Opt::Long("help") => result.help = true,
            // Unknown option
            opt => return Err(getargs::Error::UnknownOpt(opt)),
        }
    }

    Ok(result)
}

pub mod fib {
    use std::mem::replace;

    fn validate_input(nth: isize) -> Result<bool, &'static str> {
        if nth >= 0 && nth < 186 {
            Ok(true)
        } else if nth > 185 {
            Err("Number entered is too big!")
        } else {
            Err("Number cannot be negative!")
        }
    }

    /// Calculate n'th Fibonacci number
    pub fn nth_fibonacci(nth: isize) -> Result<u128, &'static str> {
        validate_input(nth)?;

        let mut f0: u128 = 0;
        let mut f1: u128 = 1;

        for _ in 0..nth {
            let f2 = f0 + f1;
            f0 = replace(&mut f1, f2);
        }
        Ok(f0)
    }

    /// Return a vector of Fibonacci numbers up to n'th number
    pub fn fibonacci_to_nth(nth: isize) -> Result<Vec<u128>, &'static str> {
        validate_input(nth)?;

        let mut v: Vec<u128> = Vec::new();
        let mut f0: u128 = 0;
        let mut f1: u128 = 1;

        v.push(f0);

        for _ in 0..nth {
            let f2 = f0 + f1;
            f0 = replace(&mut f1, f2);
            v.push(f0);
        }
        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_fibonacci() {
        assert_eq!(fib::nth_fibonacci(7), Ok(13));
    }

    #[test]
    fn test_fibonacci_to_nth() {
        let v = vec![0, 1, 1, 2, 3, 5, 8, 13];
        assert_eq!(fib::fibonacci_to_nth(7), Ok(v));
    }
}
