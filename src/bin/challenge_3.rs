//! Challenge 3 - Largest Prime factor
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//!
//! What is the largest prime factor of the number 600851475143 ?

extern crate euler;
use euler::primes;

fn main() {
    let n: u64 = 600851475143;
    let limit = (n as f64).sqrt() as usize;
    let prime_list = primes(limit);

    let mut factors = vec![];
    for prime in prime_list {
        if n % (prime as u64) == 0 {
            factors.push(prime)
        }
    }

    let biggest_factor = *factors.iter().max().unwrap();
    println!("{:?}", biggest_factor);
    assert_eq!(biggest_factor, 6857);

}
