impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let max_possible_citations = citations.len();
        let mut citations_map: Vec<usize> = vec![0; max_possible_citations + 1];
        for citation in citations.into_iter() {
            unsafe {
                *citations_map.get_unchecked_mut(max_possible_citations.min(citation as usize)) +=
                    1;
            }
        }
        let result = citations_map[max_possible_citations];
        for i in (0..max_possible_citations).rev() {
            if citations_map[i + 1] > i {
                return (i + 1) as i32;
            }
            citations_map[i] += citations_map[i + 1];
        }

        result as i32
    }

    pub fn h_index_first(citations: Vec<i32>) -> i32 {
        let mut citations_map: Vec<usize> = vec![0; citations.len()];
        let max_possible_citations = citations.len() - 1;
        for citation in citations.into_iter() {
            if citation <= 0 {
                continue;
            }
            let citation = max_possible_citations.min(citation as usize - 1);
            unsafe {
                *citations_map.get_unchecked_mut(citation) += 1;
            }
        }
        eprintln!("{:?}", citations_map.clone());
        citations_map.into_iter()
                     .rev()
                     .zip((1..=max_possible_citations + 1).rev())
                     .filter(|&(x, _)| x != 0)
                     .flat_map(|(n, x)| std::iter::repeat(x).take(n))
                     .zip(1..)
                     .take_while(|&(x, n)| x >= n)
                     .count() as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work() {
        let result = Solution::h_index(vec![3, 0, 6, 1, 5]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_index() {
        let result = Solution::h_index(vec![1, 3, 1]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_two_index() {
        let result = Solution::h_index(vec![0, 1]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_two_equal_indexes() {
        let result = Solution::h_index(vec![1, 1]);
        assert_eq!(result, 1);
    }
}
