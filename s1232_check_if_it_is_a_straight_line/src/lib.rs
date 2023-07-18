impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let dif_y = coordinates[1][1] - coordinates[0][1];
        let dif_x = coordinates[1][0] - coordinates[0][0];
        let zero_x = coordinates[0][0];
        let zero_y = coordinates[0][1];
        coordinates.into_iter()
                   .skip(2)
                   .all(|points| (points[0] - zero_x) * dif_y == (points[1] - zero_y) * dif_x)
    }
}

pub struct Solution;
