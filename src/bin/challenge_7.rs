//! Problem 7 - 10001st prime
//!
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can
//! see that the 6th prime is 13.
//!
//! What is the 10 001st prime number?

extern crate euler;

fn main() {
    let primes = euler::primes(1_000_000);

    // take one because "nth" starts from 0
    let prime = *primes.get(10001 - 1).unwrap();

    assert_eq!(prime, 104743);
    println!("{}", prime);
}
