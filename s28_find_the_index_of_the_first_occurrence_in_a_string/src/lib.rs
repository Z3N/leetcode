impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
    haystack.find(&needle).unwrap_or(-1_i32 as usize) as i32
    }
}
pub struct Solution;