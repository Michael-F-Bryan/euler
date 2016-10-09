//! Challenge 4 - Largest Palindrome Number
//!
//! A palindromic number reads the same both ways. The largest palindrome
//! made from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.

extern crate euler;
use euler::is_palindrome;

fn main() {
    let mut largest_palindrome = 0;

    for i in 100..1000 {
        for j in 100..1000 {
            let res = i*j;
            if is_palindrome(res) && res > largest_palindrome {
                largest_palindrome = res;
            }
        }
    }

    println!("{}", largest_palindrome);
}
