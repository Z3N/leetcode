impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&element| element != val);
        nums.len() as i32
    }
}

pub struct Solution;