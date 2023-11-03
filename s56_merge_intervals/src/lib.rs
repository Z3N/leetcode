use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut iter = intervals.into_iter();
        let mut current_interval = iter.next().unwrap();
        let mut result = Vec::new();
        for next_interval in iter {
            if Solution::is_overlapping(&current_interval, &next_interval) {
                current_interval = Solution::merge_intervals(&current_interval, &next_interval);
            } else {
                result.push(current_interval);
                current_interval = next_interval
            }
        }
        result.push(current_interval);
        result
    }

    fn is_overlapping(lhs: &[i32], rhs: &[i32]) -> bool {
        (rhs[1] - lhs[0]) * (lhs[1] - rhs[0]) >= 0
    }

    fn merge_intervals(lhs: &[i32], rhs: &[i32]) -> Vec<i32> {
        vec![lhs[0].min(rhs[0]), lhs[1].max(rhs[1])]
    }
}
struct SolutionWithHeap;
impl SolutionWithHeap {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::with_capacity(intervals.len());
        for interval in intervals {
            heap.push((Reverse(interval[0]), interval[1]));
        }
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut current_interval = heap.pop().unwrap();
        while !heap.is_empty() {
            let next_interval = heap.pop().unwrap();
            if SolutionWithHeap::is_overlapping(current_interval, next_interval) {
                current_interval =
                    SolutionWithHeap::merge_intervals(current_interval, next_interval);
            } else {
                result.push(vec![current_interval.0 .0, current_interval.1]);
                current_interval = next_interval
            }
        }
        result.push(vec![current_interval.0 .0, current_interval.1]);
        result
    }

    fn is_overlapping((left, right): (Reverse<i32>, i32),
                      (left_next, right_next): (Reverse<i32>, i32))
                      -> bool {
        (right_next - left.0) * (right - left_next.0) >= 0
    }

    fn merge_intervals((left, right): (Reverse<i32>, i32),
                       (left_next, right_next): (Reverse<i32>, i32))
                       -> (Reverse<i32>, i32) {
        (Reverse(left.0.min(left_next.0)), right.max(right_next))
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]);
        result.sort();
        let mut answer = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        answer.sort();
        assert_eq!(result, answer);
    }

    #[test]
    fn it_works_too() {
        let mut result = Solution::merge(vec![vec![1, 4], vec![4, 5]]);
        result.sort();
        let mut answer = vec![vec![1, 5]];
        answer.sort();
        assert_eq!(result, answer);
    }

    #[test]
    fn it_doesnt_work() {
        let result = Solution::merge(vec![vec![2, 3],
                                          vec![4, 5],
                                          vec![6, 7],
                                          vec![8, 9],
                                          vec![1, 10]]);
        assert_eq!(result, vec![vec![1, 10]]);
    }
    #[test]
    fn it_doesnt_work_too() {
        let result = Solution::merge(vec![vec![1, 4], vec![0, 2], vec![3, 5]]);
        assert_eq!(result, vec![vec![0, 5]]);
    }
}
