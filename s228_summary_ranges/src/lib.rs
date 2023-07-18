impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        let mut iter = nums.into_iter();
        let mut start = if let Some(value) = iter.next() {
            value
        } else { return Vec::default() };
        let mut current = start;
        for next in iter {
            if current + 1 == next {
                current = next;
            } else {
                match current == start {
                    true => result.push(start.to_string()),
                    false => result.push(format!("{}->{}", start, current))
                }
                start = next;
                current = next;
            }
        }
        match current == start {
            true => result.push(start.to_string()),
            false => result.push(format!("{}->{}", start, current))
        }
        result
    }
}

pub struct Solution;