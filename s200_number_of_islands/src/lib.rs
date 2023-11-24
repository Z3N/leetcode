impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        Self::process_grid(grid)
    }

    fn process_grid(grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut uf = UnionFind::new(rows, cols);
        for (row, row_vec) in grid.iter().enumerate() {
            for (col, ch) in row_vec.iter().enumerate() {
                if *ch == '1' {
                    Self::connect_neighbors(&grid, &mut uf, row, col, rows, cols);
                }
            }
        }
        grid.into_iter().enumerate().fold(0, |acc, (row, row_vec)| {
                                        row_vec.into_iter().enumerate().fold(acc,
                                                                             |acc, (col, ch)| {
                                                                                 if ch == '1' && uf.is_self_root(row, col) {
                                                                                     acc + 1
                                                                                 } else {
                                                                                     acc
                                                                                 }
                                                                             })
                                    })
    }

    fn connect_neighbors(grid: &Vec<Vec<char>>, uf: &mut UnionFind, row: usize, col: usize, rows: usize, cols: usize) {
        if let Some(row_to) = row.checked_sub(rows) {
            if grid[row_to][col] == '1' {
                uf.union(row, col, row_to, col);
            }
        }
        if let Some(col_to) = col.checked_sub(1) {
            if grid[row][col_to] == '1' {
                uf.union(row, col, row, col_to);
            }
        }
        if row + 1 < rows && grid[row + 1][col] == '1' {
            uf.union(row, col, row + 1, col);
        }
        if col + 1 < cols && grid[row][col + 1] == '1' {
            uf.union(row, col, row, col + 1);
        }
    }
}
struct UnionFind {
    union: Vec<usize>,
    cols:  usize
}

impl UnionFind {
    pub fn new(rows: usize, cols: usize) -> Self {
        let union = (0..rows * cols).collect();
        Self { union, cols }
    }

    fn union(&mut self, row_from: usize, col_from: usize, row_to: usize, col_to: usize) {
        let lhs = row_from * self.cols + col_from;
        let rhs = row_to * self.cols + col_to;
        let lhs_root = self.find(lhs);
        let rhs_root = self.find(rhs);
        if lhs_root != rhs_root {
            self.union[lhs_root] = rhs_root;
        }
    }

    fn find(&mut self, root: usize) -> usize {
        let mut root = root;
        while self.union[root] != root {
            root = self.union[root];
        }
        root
    }

    fn is_self_root(&self, row: usize, col: usize) -> bool {
        self.union[row * self.cols + col] == row * self.cols + col
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = vec![vec!['1', '1', '1', '1', '0'],
                     vec!['1', '1', '0', '1', '0'],
                     vec!['1', '1', '0', '0', '0'],
                     vec!['0', '0', '0', '0', '0']];
        let result = Solution::num_islands(v);
        assert_eq!(result, 1);
    }
}
