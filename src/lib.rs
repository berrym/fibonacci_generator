pub mod fib {
    fn validate_input(nth: i64) -> Result<bool, String> {
        if nth > 0 && nth < 185 {
            Ok(true)
        } else {
            Err("Invalid input!".to_string())
        }
    }

    /// Return result of a + b = n
    #[allow(dead_code)]
    pub fn nth_fibonacci(nth: i64) -> Result<u128, String> {
        validate_input(nth)?;

        let mut a: u128 = 1;
        let mut b: u128 = 1;
        let mut n: u128 = 0;

        for number in 1..185 {
            n = a + b;
            if number == nth {
                break;
            }
            a = b;
            b = n;
        }
        Ok(n)
    }

    /// Return a vector of Fibonacci numbers up to n'th number
    #[allow(dead_code)]
    pub fn fibonacci_to_nth(nth: i64) -> Result<Vec<u128>, String> {
        validate_input(nth)?;

        let mut v: Vec<u128> = Vec::new();
        let mut a: u128 = 1;
        let mut b: u128 = 1;

        v.push(b);

        let limit = nth + 1;
        for number in 1..limit {
            let c: u128 = a + b;
            a = b;
            b = c;
            v.push(c);

            if number == nth {
                break;
            }
        }
        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_fibonacci() -> Result<(), String> {
        assert_eq!(fib::nth_fibonacci(5), Ok(13));
        Ok(())
    }

    #[test]
    fn test_fibonacci_to_nth() -> Result<(), String> {
        let v = vec![1, 2, 3, 5, 8, 13];
        assert_eq!(fib::fibonacci_to_nth(5), Ok(v));
        Ok(())
    }
}
