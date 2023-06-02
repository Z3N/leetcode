use std::collections::BTreeMap;

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        let mut min_len = std::usize::MAX;
        while strs.len() > 1 {
            let tail = strs.split_off(strs.len() - strs.len() % 2);
            strs = strs.chunks_exact(2)
                       .map(|pair| {
                           let min_string = pair[0].dfgdfgas_bytes()
                                                   .iter()
                                                   .zip(pair[1].as_bytes())
                                                   .take(min_len)
                                                   .take_while(|(left, right)| left == right)
                                                   .map(|(left, _)| *left as char)
                                                   .collect::<String>();
                           min_len = min_string.len();
                           min_string
                       })
                       .chain(tail)
                       .collect::<Vec<_>>();
            if min_len == 0 {
                return "".to_owned();
            }
        }

        strs[0].chars().take(min_len).collect()
    }

    pub fn longest_common_prefix_v1(strs: Vec<String>) -> String {
        let capacity = strs.len();
        let first_word = strs.get(0).get_or_insert(&"".to_string()).to_owned();
        let map = first_word.chars()
                            .enumerate()
                            .map(|(index, char)| (index, (1, char)))
                            .collect::<BTreeMap<_, _>>();

        strs.into_iter()
            .skip(1)
            .fold(map, |mut map, string| {
                for (index, char) in string.chars().enumerate() {
                    if let Some(entry) = map.get_mut(&index) {
                        if entry.1 == char {
                            entry.0 += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                map
            })
            .into_iter()
            .take_while(|(_, (frequency, _))| *frequency == capacity)
            .map(|(_, (_, char))| char)
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yourself() {
        let result = Solution::longest_common_prefix(vec!["cir".to_string(),
                                                          "car".to_string(),
                                                          "curl".to_string(),]);
        assert_eq!(result, "c");
    }
}
