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

extern crate euler;

use std::iter::Iterator;

fn main() {
    let triples = euler::pythag_triples(1000);

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
