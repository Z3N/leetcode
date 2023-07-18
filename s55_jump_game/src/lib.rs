impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        /*let mut max = 1;
        for num in nums.into_iter() {
            max = std::cmp::max(max - 1, num);
            if max == 0 { return false }
        }
        true*/
        nums.into_iter()
            .try_fold(1, |acc, distance| {
                if acc == 0 {
                    None
                } else {
                    Some(std::cmp::max(acc - 1, distance))
                }
            })
            .is_some()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::can_jump(vec![2, 3, 1, 1, 4]);
        assert_eq!(result, true);
    }

    #[test]
    fn it_sn_work() {
        let result = Solution::can_jump(vec![3, 2, 1, 0, 4]);
        assert_eq!(result, false);
    }

    #[test]
    fn is_zero_work() {
        let result = Solution::can_jump(vec![0]);
        assert_eq!(result, true);
    }
}
