// /// Macro to generate fibonacci series as a vector
// #[macro_export]
// macro_rules! fib {
//     ($($x:expr)?) => {{
//         let mut v: Vec<u128> = Vec::new();
//         let mut f0: u128 = 0;
//         let mut f1: u128 = 1;

//         $(
//             for _ in 0..$x {
//                 if let Some(f2) = f0.checked_add(f1) {
//                     f0 = std::mem::replace(&mut f1, f2);
//                     v.push(f0);
//                 }
//             }
//             Some(v)
//         )?
//     }}
// }

pub mod fib {
    /// Validate input is correct
    fn validate_input(nth: usize) -> Result<bool, &'static str> {
        match nth {
            0..=185 => Ok(true),
            _ => Err("Number must be 0-185!")
        }
    }

    /// Generate a vector of Fibonacci numbers
    #[inline(always)]
    fn fibonacci_generator(nth: usize) -> Option<Vec<u128>> {
        let mut v: Vec<u128> = vec![0];
        let mut f0: u128 = 0;
        let mut f1: u128 = 1;

        for _ in 0..nth {
            if let Some(f2) = f0.checked_add(f1) {
                f0 = std::mem::replace(&mut f1, f2);
                v.push(f0);
            } else {
                return None;
            }
        }
        Some(v)
    }

    /// Return a vector of Fibonacci numbers up to n'th number
    pub fn fibonacci_to_nth(nth: usize) -> Option<Vec<u128>> {
        match validate_input(nth) {
            Err(e) => {
                eprintln!("{}", e);
                None
            }
            _ => {
                if let Some(v) = fibonacci_generator(nth) {
                    Some(v)
                } else {
                    None
                }
            }
        }
    }

    /// Calculate the N'th Fibonacci number
    pub fn nth_fibonacci(nth: usize) -> Option<u128> {
        match validate_input(nth) {
            Err(e) => {
                eprintln!("{}", e);
                None
            }
            _ => {
                if let Some(v) = fibonacci_generator(nth) {
                    Some(v[v.len() - 1])
                } else {
                    None
                }
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
