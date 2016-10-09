//! Challenge 10 - Summation of Primes
//!
//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//!
//! Find the sum of all the primes below two million.

extern crate euler;


fn main() {
    let n = 2_000_000;
    let sum = prime_sum(n);
    assert_eq!(sum, 142913828922);
    println!("{}", sum);
}

fn prime_sum(n: usize) -> usize {
    euler::primes(n).iter().sum()
}

#[test]
fn prime_sum_check() {
    let should_be = 17;
    let primes = euler::primes(10);
    println!("{:?}", primes);
    let got = prime_sum(10);
    assert_eq!(got, should_be);
}
