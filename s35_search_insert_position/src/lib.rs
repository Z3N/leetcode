use std::cmp::Ordering;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut size = nums.len();
        let mut left = 0;
        let mut right = size;
        while left < right {
            let mid = left + size / 2;
            match target.cmp(&nums[mid]) {
                Ordering::Less => {
                    right = mid;
                }
                Ordering::Equal => return mid as i32,
                Ordering::Greater => {
                    left = mid + 1
                }
            }
            size = right - left;
        }
        left as i32
    }
    pub fn search_insert_my(nums: Vec<i32>, target: i32) -> i32 {
        let mut mid = nums.len() / 2;
        let mut left = 0;
        let mut right = nums.len() - 1;
        loop {
            let mid_value = unsafe { nums.get_unchecked(mid) };
            match target.cmp(mid_value) {
                Ordering::Less => {
                    if mid <= left { break }
                    right = mid - 1;
                    mid = (right - left) / 2;
                }
                Ordering::Equal => break,
                Ordering::Greater => {
                    if mid == right {
                        mid += 1;
                        break
                    }
                    left = mid + 1;
                    mid = (right + mid + 1) / 2;
                }
            }
        }
        mid as i32
    }
    pub fn search_insert_ranges(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, rest) = nums.split_at(nums.len() / 2);
        let (mut mid, mut right) = rest.split_first().unwrap();
        let mut shift = 0;
        loop {
            match target.cmp(mid) {
                std::cmp::Ordering::Less => {
                    if left.is_empty() { return shift }
                    let (left_thx_for_very_old_rustc, rest) = left.split_at(left.len() / 2);
                    left = left_thx_for_very_old_rustc;
                    let (mid_thx_for_very_old_rustc, right_thx_for_very_old_rustc) = rest.split_first().unwrap();
                    mid = mid_thx_for_very_old_rustc;
                    right = right_thx_for_very_old_rustc;
                }
                std::cmp::Ordering::Equal => return left.len() as i32 + shift,
                std::cmp::Ordering::Greater => {
                    shift += left.len() as i32 + 1;
                    if right.is_empty() { return shift; }
                    let (left_thx_for_very_old_rustc, rest) = right.split_at(right.len() / 2);
                    left = left_thx_for_very_old_rustc;
                    let (mid_thx_for_very_old_rustc, right_thx_for_very_old_rustc) = rest.split_first().unwrap();
                    mid = mid_thx_for_very_old_rustc;
                    right = right_thx_for_very_old_rustc;
                }
            }
        }
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
        let result = Solution::search_insert(vec![1, 3, 5, 6], 7);
        assert_eq!(result, 4);
    }
}
