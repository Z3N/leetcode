use std::cmp::Ordering;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        nums.sort_unstable();
        let mut first = 0;
        let end = nums.len() - 2;
        while first < end {
            let first_value = nums[first];
            if first_value > 0 { break }
            let mut second = first + 1;
            let mut third = nums.len() - 1;
            while second < third {
                let second_value = nums[second];
                let third_value = nums[third];
                match 0.cmp(&(first_value + second_value + third_value)) {
                    Ordering::Greater => second += 1,
                    Ordering::Less => third -= 1,
                    Ordering::Equal => {
                        result.push(vec![first_value, second_value, third_value]);
                        second += 1;
                        while second < third && nums[second] == second_value {
                            second += 1;
                        }
                        third -= 1;
                        while second < third && nums[third] == third_value {
                            third -= 1;
                        }
                    }
                }
            }
            first += 1;
            while first < end && nums[first] == first_value {
                first += 1;
            }
        }
        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn it_works_smt() {
        let result = Solution::three_sum(vec![-1, 0, 1, 0]);
        assert_eq!(result, vec![vec![1, -1, 0]]);
    }

    #[test]
    fn it_works_long() {
        let result = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4]);
        assert_eq!(result, vec![vec![-4, 0, 4], vec![-4, 1, 3], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, -1, 2], vec![-1, 0, 1]]);
    }
}
