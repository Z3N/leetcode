impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut capacity = 0;
        while left < right {
            capacity = capacity.max((right - left) as i32 * height[left].min(height[right]));
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        capacity
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        assert_eq!(result, 49);
    }
}
