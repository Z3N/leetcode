impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut result = String::with_capacity(s.len());
        let num_rows = num_rows as usize;
        for i in 0..num_rows {
            let mut shift = 0;
            while let Some(letter) = s.as_bytes().get(i + shift) {
                result.push(*letter as char);
                let diag_letter = num_rows - 1 - i + shift;
                if diag_letter > shift
                   && diag_letter < shift + num_rows - 1
                   && diag_letter < s.len() - 1
                {
                    result.push(s.as_bytes()[diag_letter + num_rows - 1] as char);
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
}
