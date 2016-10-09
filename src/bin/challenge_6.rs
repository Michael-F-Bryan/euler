//! Challenge 6 - Sum Square difference
//!
//! The sum of the squares of the first ten natural numbers is,
//!
//! 1^2 + 2^2 + ... + 10^2 = 385
//! The square of the sum of the first ten natural numbers is,
//!
//! (1 + 2 + ... + 10)^2 = 552 = 3025
//! Hence the difference between the sum of the squares of the first ten
//! natural numbers and the square of the sum is 3025 − 385 = 2640.
//!
//! Find the difference between the sum of the squares of the first one
//! hundred natural numbers and the square of the sum.

#![feature(inclusive_range_syntax)]


fn main() {
    let d = sum_square_difference(100);
    assert_eq!(d, 25164150);
    println!("{}", d);
}

fn sum_square_difference(n: usize) -> usize {
    let sum: usize = (1...n).sum();
    let sum_square = sum.pow(2);
    let square_sum: usize = (1...n).map(|i| i * i).sum();
    sum_square - square_sum
}

#[test]
fn check_provided_sum_square() {
    let should_be = 2640;
    let got = sum_square_difference(10);
    assert_eq!(got, should_be);
}
