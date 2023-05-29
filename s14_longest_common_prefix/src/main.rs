use std::collections::BTreeMap;

fn main() {
    println!("Hello, world!");
}


struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let capacity = strs.len();
        let first_word = strs.get(0).get_or_insert(&"".to_string()).to_owned();
        let map = first_word.chars().enumerate().map(|(index, char)| (index, (1, char)))
            .collect::<BTreeMap<_,_>>();

        strs.into_iter()
            .skip(1)
            .fold(map,
                              |mut map, string| {
                                  for (index, char) in string.chars().enumerate() {
                                      if let Some(entry) = map.get_mut(&index) {
                                          if entry.1 == char {
                                              entry.0 += 1;
                                          } else {
                                              break
                                          }
                                      } else {
                                          break
                                      }
                                  }
                                  map
                              }
        ).into_iter()
            .take_while(|(_, (frequency, _))| *frequency == capacity)
            .map(|(_, (_, char))| char).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yourself() {
        let result = Solution::longest_common_prefix(vec!["cir".to_string(),"car".to_string()]);
        assert_eq!(result, "c");
    }
}