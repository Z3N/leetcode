impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.chars().rev()
         .skip_while(|char| !char.is_ascii_alphabetic())
         .take_while(|char| char.is_ascii_alphabetic())
         .count() as i32
    }
}

pub struct Solution;

