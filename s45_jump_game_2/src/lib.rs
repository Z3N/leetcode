impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut available_steps = 0;
        let mut jumps = 0;
        let mut max_steps_founded = 0;
        let len = nums.len();
        for (current_position, steps) in nums.into_iter().enumerate().take(len - 1) {
            max_steps_founded = max_steps_founded.max(steps as usize + current_position);
            if available_steps == current_position {
                available_steps = max_steps_founded;
                jumps += 1;
            }
        }
        jumps
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::jump(vec![0]);
        assert_eq!(result, 0);
    }
}
