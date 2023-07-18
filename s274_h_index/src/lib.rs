use std::cmp::Ordering;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {

        Self::quick_selection(citations)
    }
    fn quick_selection(mut citations: Vec<i32>) -> i32 {
        let mut left = 0;
        let len = citations.len();
        let mut right = len - 1;
        let mut last_good = 0;
        while left < right {
            let partition_point = Self::partition(&mut citations[left..=right]);
            let bigger_len = (len - partition_point) as i32;
            match bigger_len.cmp(&citations[partition_point]) {
                Ordering::Less => {
                    last_good = partition_point as i32;
                    right = partition_point - 1;
                }
                _ => {
                    left = partition_point + 1;
                }
            }
        }
        last_good
    }

    fn partition(citations: &mut [i32]) -> usize {
        let len = citations.len();
        let mut less = 0;
        let mut greater = len;
        let pivot = len / 2;
        let mut equal = 1;
        citations.swap(less, pivot);
        while equal < greater {
            match citations[equal].cmp(&citations[less]) {
                std::cmp::Ordering::Less => {
                    citations.swap(less, equal);
                    less += 1;
                    equal += 1;
                }
                std::cmp::Ordering::Equal => equal += 1,
                std::cmp::Ordering::Greater => {
                    citations.swap(greater - 1, equal);
                    greater -= 1;
                }
            }
        }
        equal
    }
}

pub struct Solution;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::h_index(vec![3, 0, 6, 1, 5]);
        assert_eq!(result, 3);
    }
}
