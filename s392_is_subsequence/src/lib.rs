impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut str_pointer = 0;
        let mut substr_pointer = 0;
        let s = s.into_bytes();
        let t = t.into_bytes();
        let substr_len = s.len();
        let str_len = t.len();

        while str_pointer < str_len && substr_pointer < substr_len {
            if t[str_pointer] == s[substr_pointer] {
                substr_pointer += 1;
            }
            str_pointer += 1;
        }
        substr_pointer == substr_len
    }
}

pub struct Solution;