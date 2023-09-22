use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        if !Self::is_squares_valid(&board) {
            return false;
        }
        let mut set = HashSet::with_capacity(9);

        for col in 0..9 {
            for row in 0..9 {
                if board[row][col].is_ascii_digit() && !set.insert(board[row][col]) {
                    return false;
                }
            }
            set.clear();
        }

        for row in board.into_iter() {
            if row.into_iter()
                  .filter(|d| d.is_ascii_digit())
                  .any(|d| !set.insert(d))
            {
                return false;
            }
            set.clear();
        }
        true
    }

    fn is_squares_valid(board: &[Vec<char>]) -> bool {
        let square_coords: [(usize, usize); 9] = [(0, 0),
                                                  (0, 3),
                                                  (0, 6),
                                                  (3, 0),
                                                  (3, 3),
                                                  (3, 6),
                                                  (6, 0),
                                                  (6, 3),
                                                  (6, 6)];
        let mut set = HashSet::with_capacity(9);
        for (x, y) in square_coords.into_iter() {
            for x in x..x + 3 {
                if board[x].iter()
                           .skip(y)
                           .take(3)
                           .filter(|d| d.is_ascii_digit())
                           .any(|d| !set.insert(*d))
                {
                    return false;
                }
            }
            set.clear();
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result =
            Solution::is_valid_sudoku(vec![vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                                           vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                                           vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                                           vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                                           vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                                           vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                                           vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                                           vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                                           vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']]);
        assert_eq!(result, true);
    }

    #[test]
    fn it_should_not_work() {
        let result =
            Solution::is_valid_sudoku(vec![vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                                           vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                                           vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                                           vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                                           vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                                           vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                                           vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                                           vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                                           vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']]);
        assert_eq!(result, false);
    }

    #[test]
    fn it_not_work() {
        let result =
            Solution::is_valid_sudoku(vec![vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
                                           vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
                                           vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
                                           vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
                                           vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
                                           vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
                                           vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
                                           vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
                                           vec!['.', '.', '4', '.', '.', '.', '.', '.', '.']]);
        assert_eq!(result, false);
    }

    #[test]
    fn god_no_pls_no() {
        let result =
            Solution::is_valid_sudoku(vec![vec!['.', '.', '.', '.', '.', '.', '5', '.', '.'],
                                           vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                                           vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                                           vec!['9', '3', '.', '.', '2', '.', '4', '.', '.'],
                                           vec!['.', '.', '7', '.', '.', '.', '3', '.', '.'],
                                           vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                                           vec!['.', '.', '.', '3', '4', '.', '.', '.', '.'],
                                           vec!['.', '.', '.', '.', '.', '3', '.', '.', '.'],
                                           vec!['.', '.', '.', '.', '.', '5', '2', '.', '.']]);
        assert_eq!(result, false);
    }
}
