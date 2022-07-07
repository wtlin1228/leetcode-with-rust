#![allow(dead_code)]

// 973. K Closest Points to Origin
// https://leetcode.com/problems/k-closest-points-to-origin/

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn get_length_square(&self) -> i32 {
        self.x.pow(2) + self.y.pow(2)
    }

    fn to_vec_i32(&self) -> Vec<i32> {
        vec![self.x, self.y]
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_length_square().cmp(&other.get_length_square())
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.get_length_square() == other.get_length_square()
    }
}

// Time Complexity: O(N log K), where N is the length of points, K is k
// Space Complexity: O(K), where K is k
fn k_closest_with_max_heap(points: Vec<Point>, k: i32) -> Vec<Point> {
    // This heap will only hold up to k elements
    let mut heap = BinaryHeap::new();

    for point in points {
        if heap.len() < k as usize {
            heap.push(point);
        } else {
            let peek = heap.peek().unwrap();
            if peek.get_length_square() > point.get_length_square() {
                heap.pop();
                heap.push(point);
            }
        }
    }

    heap.into_vec()
}

fn internal_k_closest(points: Vec<Point>, k: i32) -> Vec<Point> {
    k_closest_with_max_heap(points, k)
}

pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut input: Vec<Point> = Vec::new();
    for point in points {
        input.push(Point::new(point[0], point[1]));
    }

    internal_k_closest(input, k)
        .iter()
        .map(|point| point.to_vec_i32())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            k_closest(vec![vec![10, 10], vec![3, 3], vec![2, 2], vec![1, 1]], 2),
            vec![vec![2, 2], vec![1, 1]]
        );

        assert_eq!(
            k_closest(vec![vec![10, 10], vec![-3, 3], vec![-2, 2], vec![1, 1]], 2),
            vec![vec![-2, 2], vec![1, 1]]
        );
    }
}
