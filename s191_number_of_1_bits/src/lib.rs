impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        n.count_ones() as i32
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
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
