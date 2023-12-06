use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let board_len = board.len();
        let pos: i32 = 1;
        queue.push_back((pos, 0));
        while let Some((pos, step)) = queue.pop_front() {
            if pos as usize == board_len * board_len {
                return step;
            }
            for cur_pos in 1..7 {
                if cur_pos + pos > (board_len * board_len) as i32 {
                    break;
                }
                let next_pos = Self::get_next_pos(pos + cur_pos, &board);
                if visited.insert(next_pos) {
                    queue.push_back((next_pos, step + 1));
                }
            }
        }
        -1
    }

    fn get_next_pos(pos: i32, board: &Vec<Vec<i32>>) -> i32 {
        let (row, col) = Self::from_pos(pos, board.len());
        if board[row][col] != -1 {
            board[row][col]
        } else {
            pos
        }
    }

    fn from_pos(pos: i32, board_len: usize) -> (usize, usize) {
        let pos = pos as usize;
        let row = board_len - (pos - 1) / board_len - 1;
        let col = if (board_len - row) % 2 != 0 {
            (pos - 1) % board_len
        } else {
            board_len - (pos - 1) % board_len - 1
        };
        (row, col)
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::from_pos(1, 3);
        assert_eq!(result, (2, 0));
        let result = Solution::from_pos(2, 3);
        assert_eq!(result, (2, 1));
        let result = Solution::from_pos(3, 3);
        assert_eq!(result, (2, 2));
        let result = Solution::from_pos(4, 3);
        assert_eq!(result, (1, 2));
        let result = Solution::from_pos(5, 3);
        assert_eq!(result, (1, 1));
        let result = Solution::from_pos(6, 3);
        assert_eq!(result, (1, 0));
        let result = Solution::from_pos(7, 3);
        assert_eq!(result, (0, 0));
        let result = Solution::from_pos(8, 3);
        assert_eq!(result, (0, 1));
        let result = Solution::from_pos(9, 3);
        assert_eq!(result, (0, 2));
        let result = Solution::from_pos(10, 6);
        assert_eq!(result, (4, 2));
    }
    #[test]
    fn it_doesnt_work() {
        let vec = vec![vec![-1, -1, -1, -1, -1, -1],
                       vec![-1, -1, -1, -1, -1, -1],
                       vec![-1, -1, -1, -1, -1, -1],
                       vec![-1, 35, -1, -1, 13, -1],
                       vec![-1, -1, -1, -1, -1, -1],
                       vec![-1, 15, -1, -1, -1, -1]];
        let result = Solution::snakes_and_ladders(vec);
        assert_eq!(result, 4);
    }
}
