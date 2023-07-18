impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (0..n)
            .fold((1, 0), |(fib, prev_fib), _| (fib + prev_fib, fib))
            .0
    }
}
pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::climb_stairs(8);
        dbg!(result);
        assert_eq!(result, 34);
    }
}
