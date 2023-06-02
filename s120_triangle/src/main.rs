use std::cmp::min;

fn main() {
    println!("Hello, world!");
    println!(
        "{}",
        Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
    )
}

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        triangle
            .into_iter()
            .rev()
            .reduce(|acc, row| {
                row.into_iter()
                    .zip(acc.windows(2))
                    .map(|(previous, pair)| pair[0].min(pair[1]) + previous)
                    .collect::<Vec<i32>>()
            })
            .unwrap()[0]
    }
    pub fn minimum_total_for(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle: Vec<Vec<i32>> = triangle;
        for i in (1..triangle.len()).rev() {
            for j in 0..i {
                triangle[i - 1][j] += min(triangle[i][j], triangle[i][j + 1]);
            }
        }
        triangle[0][0]
    }
}

struct Solution;
