//! A crate containing all the utility functions needed to work on
//! Project Euler challenges.


#![feature(test)]

// Add some more lints
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unused_import_braces, unused_qualifications)]

// Imported crates
#[macro_use]
extern crate log;
extern crate test;
extern crate regex;

mod primes;

use std::cmp::{max, min};

pub use primes::{ErosthenesSeive, primes};


/// Using the frontier method, find the maximum path through a triangle
/// summing the values of each element traversed as you go.
///
/// This uses the "frontier method" as described in
/// http://stackoverflow.com/a/8002423
pub fn frontier_reduce(mut triangle: Vec<Vec<usize>>) -> usize {
    while triangle.len() > 1 {
        let bottom = triangle.pop().unwrap();
        let top = triangle.pop().unwrap();

        let mut next = vec![];
        for (i, value) in top.iter().enumerate() {
            // Choose whether left or right is best
            let left = bottom[i];
            let right = bottom[i + 1];
            let n = max(left, right);
            next.push(value + n);
        }
        debug!("{:?}", next);
        triangle.push(next);
    }

    triangle[0][0]
}


/// An iterator over the fibonacci numbers.
#[derive(Debug, Copy, Clone)]
pub struct Fibonacci {
    first: usize,
    second: usize,
}

impl Fibonacci {
    /// Create a new fibonacci
    pub fn new() -> Fibonacci {
        Self::default()
    }
}

impl Default for Fibonacci {
    fn default() -> Fibonacci {
        Fibonacci {
            first: 1,
            second: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let ret = self.first;
        self.first = self.second;
        self.second = ret + self.first;
        Some(ret)
    }
}


/// Check if a number is a palindrome.
pub fn is_palindrome(n: usize) -> bool {
    let as_str = format!("{}", n);
    let forwards = as_str.chars();
    let backwards = as_str.chars().rev();
    forwards.zip(backwards).all(|(a, b)| a == b)
}


/// Calculate the greatest common divisor of two numbers.
pub fn greatest_common_divisor(a: usize, b: usize) -> usize {
    let (mut a, mut b) = (min(a, b), max(a, b));

    while b > 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

/// Calculate the lowest common multiple of two numbers.
pub fn lowest_common_multiple(a: usize, b: usize) -> usize {
    a * b / greatest_common_divisor(a, b)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frontier_reduce_using_sample() {
        let triangle = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
        let should_be = 23;
        let got = frontier_reduce(triangle);
        assert_eq!(should_be, got);
    }

    #[test]
    fn basic_fib() {
        let should_be = vec![1, 1, 2, 3, 5, 8, 13];
        let fib = Fibonacci::new();
        let got: Vec<_> = fib.take(should_be.len()).collect();
        assert_eq!(got, should_be);
    }

    #[test]
    fn odd_palindrome_check() {
        let n = 12321;
        assert!(is_palindrome(n));
    }

    #[test]
    fn even_palindrome_check() {
        let n = 123321;
        assert!(is_palindrome(n));
    }

    #[test]
    fn not_a_palindrome() {
        let n = 1234;
        assert!(!is_palindrome(n));
    }

    #[test]
    fn basic_gcm() {
        let (a, b) = (10, 8);
        let should_be = 2;
        let got = greatest_common_divisor(a, b);
        assert_eq!(got, should_be);
    }

    #[test]
    fn basic_lcm() {
        let a = 5;
        let b = 6;
        let should_be = 30;
        let got = lowest_common_multiple(a, b);
        assert_eq!(got, should_be);
    }
}
