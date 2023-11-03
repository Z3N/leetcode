use std::cmp::Reverse;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut start = 0;
        let end = intervals.len();
        while start < end && intervals[start][1] < new_interval[0] {
            start += 1;
        }
        if start < end && Self::is_overlapping(&intervals[start], &new_interval) {
            let mut merged_interval = start;
            intervals[merged_interval] = Self::merge_intervals(&intervals[merged_interval], &new_interval);
            start += 1;
            while start < end && Self::is_overlapping(&intervals[merged_interval], &intervals[start]) {
                intervals[merged_interval] = Self::merge_intervals(&intervals[merged_interval], &intervals[start]);
                start += 1;
            }
            while start < end {
                merged_interval += 1;
                intervals.swap(merged_interval, start);
                start += 1;
            }
            intervals.truncate(intervals.len() - (intervals.len() - merged_interval - 1));
        } else {
            intervals.insert(start, new_interval);
        }
        intervals
    }

    fn is_overlapping(lhs: &[i32], rhs: &[i32]) -> bool {
        (rhs[1] - lhs[0]) * (lhs[1] - rhs[0]) >= 0
    }

    fn merge_intervals(lhs: &[i32], rhs: &[i32]) -> Vec<i32> {
        vec![lhs[0].min(rhs[0]), lhs[1].max(rhs[1])]
    }
}

struct Solution;

impl SolutionHeap {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(intervals.len() + 1);
        let mut heap = std::collections::BinaryHeap::with_capacity(intervals.len() + 1);
        heap.push((Reverse(new_interval[0]), new_interval[1]));
        for interval in intervals {
            heap.push((Reverse(interval[0]), interval[1]));
        }
        let mut current_interval = heap.pop().unwrap();
        while let Some(interval) = heap.pop() {
            if Self::is_overlapping(current_interval, interval) {
                current_interval = Self::merge_intervals(current_interval, interval);
            } else {
                result.push(vec![current_interval.0 .0, current_interval.1]);
                current_interval = interval;
            }
        }
        result.push(vec![current_interval.0 .0, current_interval.1]);
        result
    }

    fn is_overlapping((left, right): (Reverse<i32>, i32), (left_next, right_next): (Reverse<i32>, i32)) -> bool {
        (right_next - left.0) * (right - left_next.0) >= 0
    }

    fn merge_intervals((left, right): (Reverse<i32>, i32), (left_next, right_next): (Reverse<i32>, i32)) -> (Reverse<i32>, i32) {
        (Reverse(left.0.min(left_next.0)), right.max(right_next))
    }
}

struct SolutionHeap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]);
        assert_eq!(result, vec![vec![1, 5], vec![6, 9]]);
    }

    #[test]
    fn it_works_too() {
        let result = Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![4, 8]);
        assert_eq!(result, vec![vec![1, 2], vec![3, 10], vec![12, 16]]);
    }
    #[test]
    fn it_void() {
        let result = Solution::insert(vec![], vec![5, 7]);
        assert_eq!(result, vec![vec![5, 7]]);
    }
    #[test]
    fn it_smaller() {
        let result = Solution::insert(vec![vec![1, 5]], vec![0, 0]);
        assert_eq!(result, vec![vec![0, 0], vec![1, 5]]);
    }
    #[test]
    fn it_between() {
        let result = Solution::insert(vec![vec![3, 5], vec![12, 15]], vec![6, 6]);
        assert_eq!(result, vec![vec![3, 5], vec![6, 6], vec![12, 15]]);
    }
}
