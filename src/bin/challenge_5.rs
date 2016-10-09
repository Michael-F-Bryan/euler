//! Problem 5 - Smallest Multiple
//!
//! 2520 is the smallest number that can be divided by each of the numbers
//! from 1 to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of
//! the numbers from 1 to 20?
#![feature(inclusive_range_syntax)]

extern crate euler;

use euler::lowest_common_multiple;


fn main() {
    let smallest = (1...20).fold(1, lowest_common_multiple);
    assert_eq!(smallest, 232792560);
    println!("{}", smallest);
}
