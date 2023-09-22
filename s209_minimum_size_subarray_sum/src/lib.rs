impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut answer = usize::MAX;
        let mut sum = vec![0; nums.len() + 1];
        for i in 1..sum.len() {
            sum[i] = sum[i - 1] + nums[i - 1];
        }
        for i in 0..sum.len() {
            let search_for = target + sum[i];
            let current_slice = &sum[i..];
            let bound = current_slice.binary_search(&search_for)
                                     .unwrap_or_else(|e| e);
            if bound < current_slice.len() && current_slice[bound] - sum[i] >= target {
                answer = answer.min(bound);
            }
        }
        if answer == usize::MAX {
            0
        } else {
            answer as i32
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::min_sub_array_len(7, vec![2, 3, 2, 2, 4, 3]);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works_question_mark() {
        let result = Solution::min_sub_array_len(4, vec![1, 4, 4]);
        assert_eq!(result, 1);
    }

    #[test]
    fn it_should_work() {
        let result = Solution::min_sub_array_len(11, vec![1, 2, 3, 4, 5]);
        assert_eq!(result, 3);
    }
}
