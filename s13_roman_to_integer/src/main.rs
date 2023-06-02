fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut roman_array = (s.as_bytes(), 0u32);
        let mut result = 0;
        loop {
            roman_array = match roman_array {
                ([first, second, third, remain @ ..], previous)
                    if first == second && second == third =>
                {
                    let arabic = Solution::map_roman_to_int(*first) * 3;
                    if arabic > previous {
                        result -= previous;
                    } else {
                        result += previous;
                    }
                    (remain, arabic)
                }
                ([first, second, remain @ ..], previous) if first == second => {
                    let arabic = Solution::map_roman_to_int(*first) * 2;
                    if arabic > previous {
                        result -= previous;
                    } else {
                        result += previous;
                    }
                    (remain, arabic)
                }
                ([first, remain @ ..], previous) => {
                    let arabic = Solution::map_roman_to_int(*first);
                    if arabic > previous {
                        result -= previous;
                    } else {
                        result += previous;
                    }
                    (remain, arabic)
                }
                ([], previous) => {
                    result += previous;
                    return result as i32;
                }
            }
        }
    }

    fn map_roman_to_int(roman: u8) -> u32 {
        match roman {
            b'I' => 1,
            b'V' => 5,
            b'X' => 10,
            b'L' => 50,
            b'C' => 100,
            b'D' => 500,
            b'M' => 1000,
            _ => 0,
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yourself() {
        let result = Solution::roman_to_int("DVIII".to_owned());
        assert_eq!(result, 508);
    }
}
