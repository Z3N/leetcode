use std::collections::HashMap;

use rand::prelude::*;

struct RandomizedSet {
    hash_map: HashMap<i32, usize>,
    data:     Vec<i32>,
    rng:      ThreadRng
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet { hash_map: HashMap::new(),
                        data:     Vec::new(),
                        rng:      thread_rng() }
    }

    fn insert(&mut self, val: i32) -> bool {
        if let std::collections::hash_map::Entry::Vacant(e) = self.hash_map.entry(val) {
            e.insert(self.data.len());
            self.data.push(val);
            true
        } else {
            false
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.hash_map.contains_key(&val) {
            return false;
        }
        let removed = self.hash_map.remove(&val).unwrap();
        if removed == self.data.len() - 1 {
            self.data.pop();
        } else {
            let replaced = self.data.pop().unwrap();
            self.data[removed] = replaced;
            self.hash_map
                .entry(replaced)
                .and_modify(|val| *val = removed);
        }
        self.hash_map.remove(&val);
        true
    }

    fn get_random(&mut self) -> i32 {
        let random_picked = self.rng.gen_range(0..self.data.len());
        self.data[random_picked]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
