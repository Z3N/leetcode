use std::cmp::Ordering;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (shortest, longest) = match a.len().cmp(&b.len()) {
            Ordering::Less => (a, b),
            _ => (b, a)
        };
        let shortest_len = shortest.len();
        let mut carried = b'0';
        let mut vec = Vec::with_capacity(longest.len() + 1);
        vec.push(b'0');
        vec.extend(longest.into_bytes().into_iter());
        for (dest, source) in vec.iter_mut().rev().zip(shortest.into_bytes().into_iter().rev()) {
            match source.cmp(dest) {
                Ordering::Equal if source == b'1' => {
                    *dest &= carried;
                    carried = b'1';
                }
                Ordering::Equal => {
                    *dest |= carried;
                    carried = b'0';
                }
                _ => {
                    *dest = (carried ^ b'1') | b'0';
                    carried &= b'1';
                }
            }
        }
        for dest in vec.iter_mut().rev().skip(shortest_len) {
            *dest = (carried ^ *dest) | b'0';
            if *dest == b'1' { break }
        }
        if vec[0] == b'0' {
            unsafe { std::str::from_utf8_unchecked(&vec[1..]) }.into()
        } else {
            unsafe { String::from_utf8_unchecked(vec) }
        }
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
        let result = Solution::add_binary("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string(), "1".to_string());
        assert_eq!(result, "111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111110");
    }

    #[test]
    fn it_works_smt() {
        let result = Solution::add_binary("0".to_string(), "0".to_string());
        assert_eq!(result, "0");
    }
}
