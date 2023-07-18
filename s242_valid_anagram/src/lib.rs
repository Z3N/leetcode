impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut array = [0_u16; 32];
        s.into_bytes().into_iter().for_each(|letter| array[(letter - b'a') as usize] += 1);
        for letter in t.into_bytes().into_iter() {
            if let Some(new_value) = array[(letter - b'a') as usize].checked_sub(1) {
                array[(letter - b'a') as usize] = new_value;
            } else {
                return false;
            }
        }
        array.into_iter().all(|counter| counter == 0)
    }
}
pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::is_anagram("rat".to_string(), "car".to_string());
        assert_eq!(result, false);
    }
}
