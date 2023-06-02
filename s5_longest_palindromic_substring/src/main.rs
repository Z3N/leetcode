fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let manacher_string = "^"
            .bytes()
            .chain(s.bytes())
            .flat_map(|char| [char, b'#'])
            .chain("$".bytes())
            .collect::<Vec<u8>>();
        let string_len = s.len();
        let mut center = 2;
        let mut max_right = 3;
        let mut palindromes = vec![0; 2 * string_len + 1];
        palindromes[center] = 1;
        let mut max_palindrome = 1;
        let mut max_palindrome_index = center;

        for index in 3..=2 * string_len {
            if index < max_right {
                let index_mirror = 2 * center - index;
                palindromes[index] = palindromes[index_mirror].min(max_right - index);
            }
            while manacher_string[index + palindromes[index] + 1]
                == manacher_string[index - palindromes[index] - 1]
            {
                palindromes[index] += 1;
            }
            if index + palindromes[index] > max_right {
                center = index;
                max_right = index + palindromes[index];
            }
            if palindromes[index] > max_palindrome {
                max_palindrome = palindromes[index];
                max_palindrome_index = index;
            }
        }
        manacher_string
            .into_iter()
            .skip(max_palindrome_index - max_palindrome + 1)
            .take(2 * max_palindrome - 1)
            .filter(|char| *char != b'#')
            .map(|char| char as char)
            .collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yourself() {
        let str = String::from("cbcbccde");
        let result = Solution::longest_palindrome(str);
        assert_eq!(result, "cbcbc");
    }
    #[test]
    fn test_doubled() {
        let str = String::from("cbbd");
        let result = Solution::longest_palindrome(str);
        assert_eq!(result, "bb");
    }
}
