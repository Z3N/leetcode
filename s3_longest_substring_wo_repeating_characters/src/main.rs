use std::collections::BTreeMap;

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: BTreeMap<char, i32> = BTreeMap::default();
        let mut lower_bound = 0;
        let mut result = 0;
        for (index, ch) in s.chars().enumerate() {
            lower_bound = lower_bound.max(map.insert(ch,index as i32).unwrap_or(-1) + 1);
            result = result.max(index as i32 - lower_bound + 1);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_only_one() {
        let str = String::from("a");
        let result = Solution::length_of_longest_substring(str);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_many() {
        let str = String::from("pwwkew");
        let result = Solution::length_of_longest_substring(str);
        assert_eq!(result, 3);
    }
    #[test]
    fn test_special() {
        let str = String::from("aabaab!bb");
        let result = Solution::length_of_longest_substring(str);
        assert_eq!(result, 3);
    }
    #[test]
    fn test_yourself() {
        let str = String::from("tmmzuxt");
        let result = Solution::length_of_longest_substring(str);
        assert_eq!(result, 5);
    }
}
