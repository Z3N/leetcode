impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let mut events = Vec::new();
        let directions = [(1, 0),
                          (0, 1),
                          (-1, 0),
                          (0, -1),
                          (1, 1),
                          (-1, -1),
                          (1, -1),
                          (-1, 1)];
        for row in 0..board.len() {
            for col in 0..board[row].len() {
                let mut count = 0;
                for (dr, dc) in directions {
                    let (r, c) = (row as i32 + dr, col as i32 + dc);
                    if let Some(value) = board.get(r as usize).and_then(|r| r.get(c as usize)) {
                        count += value;
                    }
                }
                if board[row][col] == 1 {
                    if !(2..=3).contains(&count) {
                        events.push(Event::Dead(row, col));
                    }
                } else if count == 3 {
                    events.push(Event::Born(row, col));
                }
            }
        }
        for event in events {
            match event {
                Event::Born(row, col) => board[row][col] = 1,
                Event::Dead(row, col) => board[row][col] = 0
            }
        }
    }
}

enum Event {
    Born(usize, usize),
    Dead(usize, usize)
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        Solution::game_of_life(&mut result);
        assert_eq!(result, vec![vec![0, 0, 0],
                                vec![1, 0, 1],
                                vec![0, 1, 1],
                                vec![0, 1, 0]]);
    }

    #[test]
    fn it_works_short() {
        let result = &mut vec![vec![1, 1], vec![1, 0]];
        Solution::game_of_life(result);
        assert_eq!(result, &vec![vec![1, 1], vec![1, 1]]);
    }
}
