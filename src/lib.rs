pub mod fib {
    use std::mem::replace;

    fn validate_input(nth: i64) -> Result<bool, String> {
        if nth >= 0 && nth < 186 {
            Ok(true)
        } else if nth > 185 {
            Err(String::from("Number entered is too big!"))
        } else {
            Err(String::from("Number cannot be negative!"))
        }
    }

    /// Calculate n'th Fibonacci number
    #[allow(dead_code)]
    pub fn nth_fibonacci(nth: i64) -> Result<u128, String> {
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
    #[allow(dead_code)]
    pub fn fibonacci_to_nth(nth: i64) -> Result<Vec<u128>, String> {
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
