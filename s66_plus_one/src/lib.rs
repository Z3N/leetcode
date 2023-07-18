impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for digit in digits.iter_mut().rev() {
            if *digit == 9 {
                *digit = 0;
            } else {
                *digit += 1;
                break;
            }
        }
        let first = digits.first_mut().unwrap();
        if *first == 0 {
            *first = 1;
            digits.push(0);
        }
        digits
    }
}

pub struct Solution;