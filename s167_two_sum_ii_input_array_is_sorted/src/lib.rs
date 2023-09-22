use std::cmp::Ordering;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;
        while left < right {
            match target.cmp(&(numbers[left] + numbers[right])) {
                Ordering::Less => right -= 1,
                Ordering::Equal => return vec![left as i32 + 1, right as i32 + 1],
                Ordering::Greater => left += 1
            }
        }
        unreachable!()
    }
}

struct Solution;
