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

use std::cmp::max;

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
}
