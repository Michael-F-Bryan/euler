//! Challenge 9 - Special Pythagorean Triple
//!
//! A Pythagorean triplet is a set of three natural numbers, a < b < c,
//! for which,
//!
//! a^2 + b^2 = c^2
//! For example, 32 + 42 = 9 + 16 = 25 = 52.
//!
//! There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//! Find the product abc.

#![feature(inclusive_range_syntax)]

use std::iter::Iterator;

fn main() {
    let triples = pythag_triples(500);

    let specials: Vec<_> = triples.iter()
        .filter(|&triple| {
            let &(a, b, c) = triple;
            a + b + c == 1000
        })
        .collect::<Vec<_>>();

    let special = specials.iter().next().unwrap();
    let product = special.0 * special.1 * special.2;
    assert_eq!(product, 31875000);
    println!("{:?}", product);
}


/// Extremely naive way of generating pythagorean triples.
///
/// Do not use for n > 1000!!!!111
fn pythag_triples(n: u32) -> Vec<(u32, u32, u32)> {
    let mut triples = vec![];
    for i in 1...n {
        for j in i...n {
            for k in j...n {
                if i * i + j * j == k * k {
                    triples.push((i, j, k));
                }
            }
        }
    }
    triples
}
