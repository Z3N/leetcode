impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.len() < 3 {
            return;
        }
        for row in 0..board.len() {
            Self::check_border_case(board, row, 0);
            Self::check_border_case(board, row, board[0].len() - 1);
        }
        for col in 0..board[0].len() {
            Self::check_border_case(board, 0, col);
            Self::check_border_case(board, board.len() - 1, col);
        }
        for ch in board.iter_mut().flatten() {
            *ch = if *ch == 'T' { 'O' } else { 'X' };
        }
    }

    fn check_border_case(board: &mut Vec<Vec<char>>, row: usize, col: usize) {
        if board[row][col] != 'O' {
            return;
        }
        board[row][col] = 'T';
        if let Some(row) = row.checked_sub(1) {
            Self::check_border_case(board, row, col)
        }
        if let Some(col) = col.checked_sub(1) {
            Self::check_border_case(board, row, col)
        }
        if row + 1 < board.len() {
            Self::check_border_case(board, row + 1, col);
        }
        if col + 1 < board[0].len() {
            Self::check_border_case(board, row, col + 1)
        }
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = vec![vec!['X', 'X', 'X', 'X'], vec!['X', 'O', 'O', 'X'], vec!['X', 'X', 'O', 'X'], vec!['X',
                                                                                                            'O',
                                                                                                            'X',
                                                                                                            'X']];
        Solution::solve(&mut v);
        let expected = vec![vec!['X', 'X', 'X', 'X'], vec!['X', 'X', 'X', 'X'], vec!['X', 'X', 'X', 'X'], vec!['X',
                                                                                                               'O',
                                                                                                               'X',
                                                                                                               'X']];
        assert_eq!(v, expected);
    }
    #[test]
    fn short() {
        let mut v = vec![vec!['X']];
        Solution::solve(&mut v);
        let expected = vec![vec!['X']];
        assert_eq!(v, expected);
    }
    #[test]
    fn zeroes() {
        let mut v = vec![vec!['O', 'O', 'O'], vec!['O', 'O', 'O'], vec!['O', 'O', 'O']];
        Solution::solve(&mut v);
        let expected = vec![vec!['O', 'O', 'O'], vec!['O', 'O', 'O'], vec!['O', 'O', 'O']];
        assert_eq!(v, expected);
    }
    #[test]
    fn it_should_work() {
        let mut v = vec![vec!['X', 'O', 'X', 'X'], vec!['O', 'X', 'O', 'X'], vec!['X', 'O', 'X', 'O'], vec!['O',
                                                                                                            'X',
                                                                                                            'O',
                                                                                                            'X']];
        Solution::solve(&mut v);
        let expected = vec![vec!['X', 'O', 'X', 'X'], vec!['O', 'X', 'X', 'X'], vec!['X', 'X', 'X', 'O'], vec!['O',
                                                                                                               'X',
                                                                                                               'O',
                                                                                                               'X']];
        assert_eq!(v, expected);
    }
    #[test]
    fn it_doesnt_work() {
        let mut v = vec![vec!['O', 'X', 'O'], vec!['X', 'O', 'X'], vec!['O', 'X', 'O']];
        Solution::solve(&mut v);
        let expected = vec![vec!['O', 'X', 'O'], vec!['X', 'X', 'X'], vec!['O', 'X', 'O'],];
        assert_eq!(v, expected);
    }
}
