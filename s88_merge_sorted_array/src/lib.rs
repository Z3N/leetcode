impl Solution {
    pub fn merge(to: &mut Vec<i32>, m: i32, from: &mut Vec<i32>, n: i32) {
        if n == 0 { return; }
        let mut write_to = to.len() - 1;
        let mut from_last_index = n as usize - 1;
        for to_last_index in (0..m as usize).rev() {
            while to[to_last_index] < from[from_last_index] {
                to[write_to] = from[from_last_index];
                if from_last_index == 0 {
                    return;
                }
                from_last_index -= 1;
                write_to -= 1;
            }
            to[write_to] = to[to_last_index];
            write_to -= 1;
        }
        to[0..=write_to].copy_from_slice(&from[0..=from_last_index]);
    }
}

pub struct Solution;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        Solution::merge(&mut vec![0], 0, &mut vec![1], 1);
    }
}
