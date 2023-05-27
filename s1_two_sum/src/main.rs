use std::collections::BTreeMap;

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: BTreeMap<i32, usize> = BTreeMap::default();
        for (i, value) in nums.into_iter().enumerate() {
            let compliment = target - value;
            match map.get(&compliment) {
                None => { map.insert(compliment, i); }
                Some(found_index) => { return vec![i as i32, *found_index as i32]; }
            }
        }
        Vec::default()
    }
}