impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks = blocks.into_bytes();
        let mut result = i32::MAX;
        let mut total_len = 0;
        let mut recolors = 0;
        let mut left = 0;
        let mut right = 0;
        let len = blocks.len();
        while right < len {
            if blocks[right] == b'W' {
                recolors += 1;
            }
            total_len += 1;
            right += 1;

            if total_len == k {
                result = result.min(recolors);
                if blocks[left] == b'W' {
                    recolors -= 1;
                }
                total_len -= 1;
                left += 1;
            }
        }
        result
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::minimum_recolors("BWWWBB".to_string(), 6);
        assert_eq!(result, 3);
    }
}
