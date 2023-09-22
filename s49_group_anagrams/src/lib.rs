impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = std::collections::HashMap::new();
        for anagram in strs {
            let mut bytes = anagram.chars().collect::<Vec<char>>();
            bytes.sort();
            map.entry(bytes).or_insert(Vec::new()).push(anagram);
        }
        map.into_values().collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::group_anagrams(vec![String::from("eat"),
                                                   String::from("tea"),
                                                   String::from("tan"),
                                                   String::from("ate"),
                                                   String::from("nat"),
                                                   String::from("bat"),]);
        let result: HashSet<_> = result.into_iter().flatten().collect();
        let should_be = vec![vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
                             vec!["nat".to_string(), "tan".to_string()],
                             vec!["bat".to_string()]];
        let should_be: HashSet<_> = should_be.into_iter().flatten().collect();
        assert_eq!(result, should_be);
    }
}
