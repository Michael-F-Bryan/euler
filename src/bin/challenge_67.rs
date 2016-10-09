//! By starting at the top of the triangle below and moving to adjacent
//! numbers on the row below, the maximum total from top to bottom is 23.
//!
//! 3
//! 7 4
//! 2 4 6
//! 8 5 9 3
//!
//! That is, 3 + 7 + 4 + 9 = 23.
//!
//! Find the maximum total from top to bottom in triangle.txt (right click
//! and 'Save Link/Target As...'), a 15K text file containing a triangle
//! with one-hundred rows.

#[macro_use]
extern crate log;
extern crate euler;
extern crate env_logger;

use std::fs::File;
use std::path::Path;
use std::io::Read;

use euler::frontier_reduce;

fn main() {
    env_logger::init().unwrap();

    let filename = format!("{}/data/problem_067_triangle.txt",
                           env!("CARGO_MANIFEST_DIR"));
    let triangle = parse_triangle(&filename);

    let total = frontier_reduce(triangle);
    println!("Total: {}", total);
}

fn parse_triangle(filename: &str) -> Vec<Vec<usize>> {
    let path = Path::new(filename);
    debug!("Reading from {}", path.display());
    let mut f = File::open(&path).expect("Unable to open file");

    let mut s = String::new();
    f.read_to_string(&mut s).expect("Couldn't read to string");

    let mut triangle = vec![];
    // Iterate over the lines in our string
    for line in s.lines() {
        let row: Vec<usize> = line.split(' ').map(|i| i.trim().parse().unwrap()).collect();
        triangle.push(row);
    }

    triangle
}
