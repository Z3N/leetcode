impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        loop {
            let mut sum = 0;
            while n > 0 {
                sum += (n % 10).pow(2);
                n /= 10;
            }
            match sum {
                1 => return true,
                4 => return false,
                _ => n = sum
            }
        }
    }
}

pub struct Solution;