use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let len = nums.len();
        nums.into_iter()
            .try_fold(HashSet::with_capacity(len), |mut acc, elem| {
                acc.insert(elem).then_some(acc)
            })
            .is_none()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yourself() {
        let result = Solution::contains_duplicate(vec![1, 2, 3, 1]);
        assert_eq!(result, true);
    }
}
