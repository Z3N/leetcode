use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate_window(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::with_capacity(k as usize);
        for (index, &num) in nums.iter().enumerate().take(k as usize) {
            if let Some(prev_index) = map.insert(num, index) {
                if index - prev_index <= k as usize { return true; }
            }
        }

        for index in k as usize..nums.len() {
            if let Some(prev_index) = map.insert(nums[index], index) {
                if index - prev_index <= k as usize { return true; }
            }
        }
        false
    }
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();
        for (index, num) in nums.into_iter().enumerate() {
            if let Some(prev_index) = map.insert(num, index) {
                if index - prev_index <= k as usize { return true; }
            }
        }
        false
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::contains_nearby_duplicate_window(vec![1,2,3,1,2,3], 2);
        assert_eq!(result, false);
    }
    #[test]
    fn it_not_work() {
        let result = Solution::contains_nearby_duplicate_window(vec![1,2,3,1], 3);
        assert_eq!(result, true);
    }
}
