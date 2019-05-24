pub mod fib {
    pub fn validate_input(n: u8) -> bool {
        if n == 0 || n > 184 {
            return false;
        }
        return true;
    }

    /// Return a tuple of (nth-2) + (nth-1) = n
    #[allow(dead_code)]
    pub fn nth_fibonacci(n: u8) -> (u128, u128, u128) {
        if !validate_input(n) {
            return (0, 0, 0);
        }

        let mut a: u128 = 1;
        let mut b: u128 = 1;
        let mut c: u128 = 0;

        for number in 1..185 {
            c = a + b;
            if number == n {
                break;
            };
            a = b;
            b = c;
        }
        return (a, b, c);
    }

    /// Return a vector of Fibonacci numbers up to n'th number
    #[allow(dead_code)]
    pub fn fibonacci_to_nth(n: u8) -> Vec<u128> {
        if !validate_input(n) {
            return Vec::new()
        }

        let mut v: Vec<u128> = Vec::new();
        let mut a: u128 = 1;
        let mut b: u128 = 1;

        v.push(b);

        for number in 1..n + 1 {
            let c: u128 = a + b;
            a = b;
            b = c;
            v.push(c);

            if number == n {
                break;
            }
        }
        v
    }
}
