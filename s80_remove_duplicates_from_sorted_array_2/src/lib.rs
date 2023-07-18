use std::cmp::Ordering;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut left = 0;
        let mut equal = 1;
        let mut found_abnormality = 0;
        while equal < nums.len() {
            match nums[left].cmp(&nums[equal]) {
                Ordering::Equal => {
                    if equal - left >= 2 {
                        found_abnormality = left;
                        break;
                    }
                    equal += 1;
                }
                _ => {
                    left = equal;
                    equal += 1;
                }
            }
        }
        left = found_abnormality;
        equal = found_abnormality + 1;
        let mut copy_to = found_abnormality + 1;
        while equal < nums.len() {
            match nums[left].cmp(&nums[equal]) {
                Ordering::Equal => {
                    if equal - left < 2 {
                        nums[copy_to] = nums[equal];
                        copy_to += 1;
                    }
                    equal += 1;
                }
                _ => {
                    nums[copy_to] = nums[equal];
                    left = equal;
                    copy_to += 1;
                    equal += 1;
                }
            }
        }
        copy_to as i32
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
        let result = Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3]);
        assert_eq!(result, 4);
    }
}
