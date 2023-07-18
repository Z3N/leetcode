impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
         .rev()
         .collect::<Vec<_>>()
         .join(" ")
    }
}

pub struct Solution;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
