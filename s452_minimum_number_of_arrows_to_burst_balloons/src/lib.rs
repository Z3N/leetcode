use std::collections::BinaryHeap;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut result = 1;
        let mut it = points.into_iter();
        let mut last_intersection = it.next().unwrap()[1];
        for point in it {
            if last_intersection < point[0] {
                result += 1;
                last_intersection = point[1];
            }
        }
        result
    }
}

struct SolutionHeap;

impl SolutionHeap {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut heap = BinaryHeap::with_capacity(points.len());
        for point in points {
            heap.push((point[0], point[1]));
        }
        let mut result = 1;
        let mut last_intersection = heap.pop().unwrap().0;
        while let Some((left, right)) = heap.pop() {
            if last_intersection > right {
                result += 1;
                last_intersection = left;
            }
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
        let result = Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_does_not_work() {
        let result = Solution::find_min_arrow_shots(vec![vec![4289383, 51220269],
                                                         vec![81692777, 96329692],
                                                         vec![57747793, 81986128],
                                                         vec![19885386, 69645878],
                                                         vec![96516649, 186158070],
                                                         vec![25202362, 75692389],
                                                         vec![83368690, 85888749],
                                                         vec![44897763, 112411689],
                                                         vec![65180540, 105563966],
                                                         vec![4089172, 7544908]]);
        assert_eq!(result, 4);
    }
}
