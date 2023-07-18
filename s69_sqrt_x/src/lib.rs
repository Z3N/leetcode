impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 { return 0 }
        let mut candidate = x as f64;
        loop {
            let iteration = (candidate + x as f64 / candidate) * 0.5;
            if candidate - iteration < f64::EPSILON { break }
            candidate = iteration;
        }
        candidate as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::my_sqrt(8);
        assert_eq!(result, 2);
    }
}
