fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut temp = x;
        let mut inverted: i32 = 0;
        while temp != 0 {
            inverted = inverted * 10 + temp % 10;
            temp /= 10;
        }
        x == inverted
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yourself() {
        let result = Solution::is_palindrome(-1);
        assert_eq!(result, false);
    }

    #[test]
    fn test_i32_max() {
        let result = Solution::is_palindrome(std::i32::MAX);
        assert_eq!(result, false);
    }

    #[test]
    fn test_palindrome() {
        let result = Solution::is_palindrome(121);
        assert_eq!(result, true);
    }
}