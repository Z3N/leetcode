impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let head_should_be_zero = Self::mark_for_zeroing(matrix);
        Self::fill_with_zeroes(matrix, head_should_be_zero);
    }

    fn mark_for_zeroing(matrix: &mut [Vec<i32>]) -> bool {
        let (first_row, matrix) = matrix.split_at_mut(1);
        let head_should_be_zero = first_row[0].iter().any(|e| *e == 0);
        for matrix_mut in matrix.iter_mut() {
            for i in 0..matrix_mut.len() {
                if matrix_mut[i] == 0 {
                    first_row[0][i] = 0;
                    matrix_mut[0] = 0;
                }
            }
        }
        head_should_be_zero
    }

    fn fill_with_zeroes(matrix: &mut [Vec<i32>], fill_head: bool) {
        for matrix_mut in matrix.iter_mut().skip(1) {
            if matrix_mut[0] == 0 {
                matrix_mut.fill(0)
            }
        }
        for i in 0..matrix[0].len() {
            if matrix[0][i] == 0 {
                for matrix_mut in matrix.iter_mut() {
                    matrix_mut[i] = 0;
                }
            }
        }
        if fill_head {
            matrix[0].fill(0);
        }
    }
}

struct Solution;

impl Solution_first {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        Self::zeroes_from(matrix, 0, 0);
    }

    fn zeroes_from(matrix: &mut Vec<Vec<i32>>, mut row: usize, mut col: usize) {
        if row >= matrix.len() {
            return;
        }
        if col >= matrix[0].len() {
            col = 0;
            row += 1;
        }
        for matrix_row in row..matrix.len() {
            if let Some(pos) = matrix[matrix_row][col..].iter().position(|e| *e == 0) {
                Self::zeroes_from(matrix, matrix_row, col + pos + 1);
                Self::make_cross_zero(matrix, matrix_row, col + pos);
                break;
            }
            col = 0;
        }
    }

    fn make_cross_zero(matrix: &mut Vec<Vec<i32>>, row: usize, col: usize) {
        for matrix_row in matrix.iter_mut() {
            matrix_row[col] = 0;
        }
        matrix[row].fill(0);
    }
}

struct Solution_first;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = vec![vec![1, 2, 3, 4],
                              vec![5, 0, 7, 8],
                              vec![0, 10, 11, 12],
                              vec![13, 14, 15, 0]];
        Solution::set_zeroes(&mut result);
        assert_eq!(result, vec![vec![0, 0, 3, 0],
                                vec![0, 0, 0, 0],
                                vec![0, 0, 0, 0],
                                vec![0, 0, 0, 0]]);
    }
}
