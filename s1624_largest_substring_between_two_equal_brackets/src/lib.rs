use std::collections::HashMap;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut pairs = HashMap::new();
        let mut result = -1;
        for (index, char) in s.chars().enumerate() {
            pairs.entry(char)
                 .and_modify(|&mut left_pos| result = result.max((index - left_pos - 1) as i32))
                 .or_insert(index);
        }
        result
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work() {
        let result =
            Solution::max_length_between_equal_characters("mgntdygtxrvxjnwksqhxuxtrv".to_string());
        assert_eq!(result, 18);
    }
}
