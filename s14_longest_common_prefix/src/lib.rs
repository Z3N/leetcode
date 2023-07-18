use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix_iter(str: Vec<String>) -> String {
        str.into_iter()
            .reduce(|acc, str| {
                acc.chars()
                    .zip(str.chars())
                    .take_while(|(left, right)| left == right)
                    .map(|(left, _)| left)
                    .collect()
            })
            .unwrap_or(String::default())
    }
    pub fn longest_common_prefix_combined(mut str: Vec<String>) -> String {
        let mut min_len = std::usize::MAX;
        while str.len() > 1 {
            let tail = str.split_off(str.len() - str.len() % 2);
            str = str
                .chunks_exact(2)
                .map(|pair| {
                    let min_string = pair[0]
                        .as_bytes()
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

        str[0].chars().take(min_len).collect()
    }

    pub fn longest_common_prefix_v1(str: Vec<String>) -> String {
        let capacity = str.len();
        let first_word = str.get(0).get_or_insert(&"".to_string()).to_owned();
        let map = first_word
            .chars()
            .enumerate()
            .map(|(index, char)| (index, (1, char)))
            .collect::<BTreeMap<_, _>>();

        str.into_iter()
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
        let result = Solution::longest_common_prefix_v1(vec!["cir".to_string(), "car".to_string()]);
        assert_eq!(result, "c");
    }
}
