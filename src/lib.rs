#[macro_use]
extern crate log;

use std::cmp::max;


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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frontier_reduce_using_sample() {
        let mut triangle = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
        let should_be = 23;
        let got = frontier_reduce(triangle);
        assert_eq!(should_be, got);
    }
}
