fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut result = Vec::with_capacity(s.len());
        for char in s.chars() {
            match char {
                '(' => result.push(')'),
                '[' => result.push(']'),
                '{' => result.push('}'),
                ')' | ']' | '}' if result.pop() != Some(char) => return false,
                _ => (),
            }
        }
        result.is_empty()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yourself() {
        let result = Solution::is_valid("{}".to_owned());
        assert_eq!(result, true);
    }
}
