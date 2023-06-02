fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();
        for col in 1..matrix.len() {
            for row in 0..col {
                let (top, down) = matrix.split_at_mut(col);
                std::mem::swap(&mut down[0][row], &mut top[row][col]);
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yourself() {
        let mut vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut vec);
        vec.iter().for_each(|row| println!("{:?}", row));
        //assert_eq!(result, true);
    }

    #[test]
    fn test_bigger() {
        let mut vec = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut vec);
        vec.iter().for_each(|row| println!("{:?}", row));
        //assert_eq!(result, true);
    }

    #[test]
    fn test_small() {
        let mut vec = vec![vec![1, 2], vec![3, 4]];
        Solution::rotate(&mut vec);
        vec.iter().for_each(|row| println!("{:?}", row));
        //assert_eq!(result, true);
    }
}
