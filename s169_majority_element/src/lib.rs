use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let first = nums[0];
        nums.into_iter()
            .fold((first, 1), |(candidate, counter), element| {
                match (candidate == element, counter) {
                    (true, _) => (candidate, counter + 1),
                    (_, 1) => (element, 1),
                    (_, count) => (candidate, count - 1)
                }
            })
            .0
    }
    pub fn majority_element_hash(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold(HashMap::new(), |mut acc, value| {
                acc.entry(value).and_modify(|x| *x += 1).or_insert(1);
                acc
            })
            .into_iter()
            .max_by(|(_, lvalue), (_, rvalue)| lvalue.cmp(rvalue))
            .map(|(key, _)| key)
            .unwrap_or(0)
    }
}

pub struct Solution;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::majority_element(vec![3, 2, 3, 4]);
        assert_eq!(result, 3);
    }
}
