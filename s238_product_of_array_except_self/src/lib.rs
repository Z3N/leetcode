impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        result[0] = 1;
        for i in 0..nums.len() - 1 {
            result[i + 1] = result[i] * nums[i];
        }
        let mut postfix = 1;
        for (index, num) in nums.into_iter().enumerate().rev() {
            result[index] *= postfix;
            postfix *= num;
        }
        result
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::product_except_self(vec![1, 2, 3, 4]);
        assert_eq!(result, vec![24, 12, 8, 6]);
    }
}
