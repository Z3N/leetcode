use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut dictionary = [0; (b'z' - b'a') as usize + 1];
        let mut map = HashMap::new();
        let s = s.into_bytes();
        let split = s.split(|&x| x == b' ');
        let mut pattern = pattern.into_bytes().into_iter().fuse();
        for word in split {
            if let Some(char) = pattern.next() {
                if let Some(&old_value) = map.get(&word) {
                    if char != old_value { return false }
                } else {
                    let old_value = std::mem::replace(&mut dictionary[(char - b'a') as usize], char);
                    if old_value == 0 {
                        map.insert(word, char);
                    } else {
                        return false
                    }
                }
            } else { return false }
        }
        pattern.next().is_none()
    }
    pub fn word_pattern_iter(pattern: String, s: String) -> bool {
        let mut dictionary = [0; (b'{' - b'a') as usize + 1];
        let mut map = HashMap::new();
        s.into_bytes().split(|&x| x == b' ')
         .zip(pattern.into_bytes().into_iter())
         .all(|(word, char)| {
             if let Some(&old_value) = map.get(&word) {
                 char == old_value
             } else {
                 let old_value = std::mem::replace(&mut dictionary[(char - b'a') as usize], char);
                 if old_value == 0 {
                     map.insert(word, char);
                     true
                 } else { false }
             }
         })
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn it_doesnt_work() {
        let result = Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn it_should_not_work() {
        let result = Solution::word_pattern("aaaa".to_string(), "dog cat cat dog".to_string());
        assert_eq!(result, false);
    }
}
