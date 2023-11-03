use std::collections::HashMap;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut uf = UnionFind::new(nums);
        *uf.longest_consecutive().unwrap_or(&0) as i32
    }
}

struct Solution;

struct UnionFind {
    union:  Vec<usize>,
    map:    Option<HashMap<i32, usize>>,
    weight: Vec<usize>
}

impl UnionFind {
    pub fn new(values: Vec<i32>) -> Self {
        let mut map = HashMap::with_capacity(values.len());
        let mut index = 0;
        for value in values.into_iter() {
            if map.contains_key(&value) {
                continue;
            }
            map.insert(value, index);
            index += 1;
        }
        let set_len = map.len();
        let map = Some(map);
        let union = (0..set_len).collect();
        let weight = vec![1; set_len];
        Self { union, map, weight }
    }

    fn get_root(&mut self, i: usize) -> usize {
        let mut current = self.union[i];
        if current == self.union[current] {
            return current;
        }
        let weight = self.weight[i];
        while current != self.union[current] {
            current = self.union[current];
            self.weight[current] -= weight;
        }
        self.weight[current] += weight;
        self.union[i] = current;
        current
    }

    fn union(&mut self, lhs: usize, rhs: usize) {
        let lhs_root = self.get_root(lhs);
        let rhs_root = self.get_root(rhs);
        if lhs_root == rhs_root {
            return;
        }
        let update_weight = if self.weight[lhs_root] > self.weight[rhs_root] {
            self.union[rhs_root] = lhs_root;
            rhs_root
        } else {
            self.union[lhs_root] = rhs_root;
            lhs_root
        };
        self.update_weight(update_weight);
    }

    fn update_weight(&mut self, root: usize) {
        let predecessor = self.get_root(root);
        self.weight[predecessor] += self.weight[root];
    }

    fn connect_sequences(&mut self) {
        if let Some(map) = self.map.take() {
            for (value, index) in map.iter() {
                if let Some(smaller) = map.get(&(value - 1)) {
                    self.union(*index, *smaller);
                }
                if let Some(bigger) = map.get(&(value + 1)) {
                    self.union(*index, *bigger);
                }
            }
            self.map = Some(map);
        }
    }

    pub fn longest_consecutive(&mut self) -> Option<&usize> {
        self.connect_sequences();
        self.weight.iter().max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_too() {
        let result = Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]);
        assert_eq!(result, 9);
    }
}
