impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.into_bytes();
        let mut start = 0;
        let mut finish = s.len() - 1;

        while start < finish {
            let head = s[start] ;
            if !head.is_ascii_alphanumeric() {
                start += 1;
                continue
            }
            let tail = s[finish];
            if !tail.is_ascii_alphanumeric() {
                finish -= 1;
                continue
            }
            if head | 0x20 != tail | 0x20 { return false; }
            start += 1;
            finish -= 1;
        }
        {
            let drop = s;
        }
        true
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::is_palindrome("A man, a plan, a canal: Panama".to_string());
        assert_eq!(result, true);
    }
}
