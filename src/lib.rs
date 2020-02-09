pub mod fib {
    use std::mem::replace;

    fn validate_input(nth: usize) -> Result<bool, &'static str> {
        if nth > 185 {
            Err("Number entered is too big!")
        } else {
            Ok(true)
        }
    }

    /// Return a vector of Fibonacci numbers up to n'th number
    pub fn fibonacci_to_nth(nth: usize) -> Option<Vec<u128>> {
        match validate_input(nth) {
            Err(e) => {
                println!("{}", e);
                None
            }
            _ => {
                let mut v: Vec<u128> = vec![0];
                let mut f0: u128 = 0;
                let mut f1: u128 = 1;

                for _ in 0..nth {
                    if let Some(f2) = f0.checked_add(f1) {
                        f0 = replace(&mut f1, f2);
                        v.push(f0);
                    } else {
                        return None;
                    }
                }
                Some(v)
            }
        }
    }

    /// Calculate the N'th Fibonacci number
    pub fn nth_fibonacci(nth: usize) -> Option<u128> {
        match validate_input(nth) {
            Err(e) => {
                println!("{}", e);
                None
            }
            _ => {
                let mut f0: u128 = 0;
                let mut f1: u128 = 1;

                for _ in 0..nth {
                    if let Some(f2) = f0.checked_add(f1) {
                        f0 = replace(&mut f1, f2);
                    } else {
                        return None;
                    }
                }
                Some(f0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_to_nth() {
        let v = vec![0, 1, 1, 2, 3, 5, 8, 13];
        assert_eq!(fib::fibonacci_to_nth(7), Some(v));
    }

    #[test]
    fn test_nth_fibonacci() {
        assert_eq!(fib::nth_fibonacci(7), Some(13));
    }
}
