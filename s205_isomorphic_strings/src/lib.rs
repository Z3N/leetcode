impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut dictionary_ref = [0; (b'z' - b' ') as usize + 1];
        let mut dictionary_cmp = [0; (b'z' - b' ') as usize + 1];
        s.into_bytes().into_iter().zip(t.into_bytes().into_iter())
         .all(|(s, t)| {
             let result = dictionary_ref[(s - b' ') as usize] == dictionary_cmp[(t - b' ') as usize];
             dictionary_ref[(s - b' ') as usize] = s;
             dictionary_cmp[(t - b' ') as usize] = s;
             result
         })
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::is_isomorphic("egg".to_string(), "add".to_string());
        assert!(result);
    }
}
