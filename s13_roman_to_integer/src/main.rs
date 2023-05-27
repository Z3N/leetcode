use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.into_bytes()
            .chunks(2)
/*        s.chars()
            .map(|roman| Solution::map_roman_to_int(roman))
            .collect::<Vec<i32>>()
            .chunks()*/
            //.fold(0,|acc, value|)
        /// DO WITH MATCH
        /// //https://byjus.com/maths/roman-numerals/#:~:text=These%20roman%20numerals%20are%20I,%2C%20%E2%80%A6%20till%20XX%20for%2020.
        todo!()
    }

    fn map_roman_to_int(roman: u8) -> i32 {
        match roman {
            b'I' => 1,
            b'V' => 5,
            b'X' => 10,
            b'L' => 50,
            b'C' => 100,
            b'D' => 500,
            b'M' => 1000,
            _ => 0
        }
    }
    fn combine_roman(first: i32, second: i32) -> i32{
        match first.cmp(&second) {
            Ordering::Less => second - first,
            _ => second + first
        }
    }
}

struct Solution;