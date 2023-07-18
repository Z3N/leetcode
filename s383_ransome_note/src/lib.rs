use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map_ransom = HashMap::new();
        let mut map_magazine = HashMap::new();
        ransom_note.into_bytes().into_iter()
                   .for_each(|char| {
                       map_ransom.entry(char)
                                 .and_modify(|val| *val += 1)
                                 .or_insert(1);
                   });
        magazine.into_bytes().into_iter()
                .for_each(|char| {
                    map_magazine.entry(char)
                                .and_modify(|val| *val += 1)
                                .or_insert(1);
                });

        for (key, value) in map_ransom.into_iter() {
            if let Some(in_magazine) = map_magazine.get(&key) {
                if &value > in_magazine { return false }
            } else {
                return false
            }
        }
        true
    }
}

pub struct Solution;