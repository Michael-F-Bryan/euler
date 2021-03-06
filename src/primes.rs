//! Miscellaneous functions and structs for working with prime numbers.


/// A seive of Erosthenes. Modelled as a vector of bools where the index
/// indicates whether that number is prime or not.
#[derive(Debug)]
pub struct ErosthenesSeive {
    numbers: Vec<bool>,
}

impl ErosthenesSeive {
    /// Create a seive which will find primes up to and including `n`.
    fn new(n: usize) -> ErosthenesSeive {
        if (n as f64) > 1.0 * 10e9 {
            // Because the seive stores everything in RAM instead of
            // segmenting it, we'd be allocating a Vec of approximately
            // 1*10^9 integers
            panic!("Seive size won't fit in RAM");
        }

        ErosthenesSeive { numbers: vec![true; n+1] }
    }

    /// Iterate through the seive, marking all composite numbers as false.
    pub fn solve(&mut self) {
        // Set 0 and 1 to false because they aren't prime
        self.numbers[0] = false;
        self.numbers[1] = false;

        for i in 2..self.numbers.len() {
            // Skip composite numbers
            if !self.numbers[i] {
                continue;
            }

            // Otherwise, mark the multiples of this number as false
            let mut j = 2;
            while i * j < self.numbers.len() {
                self.numbers[i * j] = false;
                j += 1;
            }
        }
    }

    /// Get the contents of the seive.
    pub fn numbers(&self) -> &[bool] {
        &self.numbers
    }

    /// Check if a number is prime.
    pub fn is_prime(&self, n: usize) -> Result<bool, String> {
        if n < self.numbers.len() {
            Ok(self.numbers[n])
        } else {
            Err(format!("Number not in seive. Seive goes up to {} but {} was requested",
                        self.numbers.len(),
                        n))
        }
    }
}


/// Generate a vector of all the primes up to and including `n`.
pub fn primes(n: usize) -> Vec<usize> {
    let mut seive = ErosthenesSeive::new(n);
    seive.solve();

    // Collect all prime numbers
    let mut v = vec![];
    for (i, is_prime) in seive.numbers().iter().enumerate() {
        if *is_prime {
            v.push(i);
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_seive() {
        let mut seive = ErosthenesSeive::new(5);
        seive.solve();
        let should_be = vec![false, false, true, true, false, true];
        assert_eq!(seive.numbers, should_be);
    }

    #[test]
    fn get_primes() {
        let should_be = vec![2, 3, 5, 7, 11, 13, 17, 19, 23];
        let got = primes(23);
        assert_eq!(got, should_be);
    }

    #[test]
    fn seive_out_of_range() {
        let mut seive = ErosthenesSeive::new(5);
        seive.solve();

        // Check an invalid number
        let got = seive.is_prime(6);
        assert!(got.is_err());

        // Check a valid number
        let got = seive.is_prime(5);
        assert_eq!(got, Ok(true));
    }

    #[bench]
    fn find_primes_up_to_1_million(b: &mut Bencher) {
        let n = 1_000_000;

        b.iter(|| primes(n));
    }
}
