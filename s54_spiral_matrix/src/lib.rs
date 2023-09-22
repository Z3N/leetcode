impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let spiral = [((0, 1), (1, 0)), ((0, -1), (-1, 0))];
        let capacity = matrix.len() * matrix[0].len();
        let mut result = Vec::with_capacity(capacity);
        let mut max_x = matrix[0].len();
        let mut max_y = matrix.len() - 1;
        let mut x = 0;
        let mut y = -1;
        //result.push(matrix[0][0]);
        for (step_xx, step_xy) in spiral.into_iter().cycle() {
            let (step_x, step_y) = step_xx;
            for _ in 0..max_x {
                x += step_x;
                y += step_y;
                result.push(matrix[x as usize][y as usize]);
                if result.len() == capacity {
                    return result;
                }
            }
            max_x -= 1;
            let (step_x, step_y) = step_xy;
            for _ in 0..max_y {
                x += step_x;
                y += step_y;
                result.push(matrix[x as usize][y as usize]);
                if result.len() == capacity {
                    return result;
                }
            }
            max_y -= 1;
        }
        result
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert_eq!(result, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }
    #[test]
    fn it_works_too() {
        let result =
            Solution::spiral_order(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]]);
        assert_eq!(result, vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
    }
}
