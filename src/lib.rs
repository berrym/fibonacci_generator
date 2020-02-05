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
    pub fn nth_fibonacci(nth: isize) -> Option<u128> {
        validate_input(nth).unwrap();

        let mut f0: u128 = 0;
        let mut f1: u128 = 1;

        for _ in 0..nth {
            let f2 = f0 + f1;
            f0 = replace(&mut f1, f2);
        }
        Some(f0)
    }

    /// Return a vector of Fibonacci numbers up to n'th number
    pub fn fibonacci_to_nth(nth: isize) -> Option<Vec<u128>> {
        validate_input(nth).unwrap();

        let mut v: Vec<u128> = Vec::new();
        let mut f0: u128 = 0;
        let mut f1: u128 = 1;

        v.push(f0);

        for _ in 0..nth {
            let f2 = f0 + f1;
            f0 = replace(&mut f1, f2);
            v.push(f0);
        }
        Some(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_fibonacci() {
        assert_eq!(fib::nth_fibonacci(7), Some(13));
    }

    #[test]
    fn test_fibonacci_to_nth() {
        let v = vec![0, 1, 1, 2, 3, 5, 8, 13];
        assert_eq!(fib::fibonacci_to_nth(7), Some(v));
    }
}
