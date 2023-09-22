impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 || s.len() <= num_rows as usize {
            return s;
        }
        let mut result = String::with_capacity(s.len());
        let num_rows = num_rows as usize;
        for i in 0..num_rows {
            let mut shift = 0;
            while let Some(letter) = s.as_bytes().get(i + shift) {
                result.push(*letter as char);
                let diag_letter = shift + num_rows * 2 - 2 - 2 * i + i;
                if diag_letter >= shift + num_rows
                   && diag_letter < shift + num_rows * 2 - 2
                   && diag_letter < s.len()
                {
                    result.push(s.as_bytes()[diag_letter] as char);
                }
                shift += (num_rows) * 2 - 2;
            }
        }
        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::convert("PAYPALISHIRING".to_string(), 3);
        assert_eq!(result, "PAHNAPLSIIGYIR".to_string());
    }

    #[test]
    fn four() {
        let result = Solution::convert("PAYPALISHIRING".to_string(), 4);
        assert_eq!(result, "PINALSIGYAHRPI".to_string());
    }

    #[test]
    fn one() {
        let result = Solution::convert("A".to_string(), 1);
        assert_eq!(result, "A".to_string());
    }

    #[test]
    fn three_rows() {
        let result = Solution::convert("ABCD".to_string(), 3);
        assert_eq!(result, "ABDC".to_string());
    }
}
