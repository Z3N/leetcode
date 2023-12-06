impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut degrees = vec![0; num_courses as usize];
        let mut descendants = vec![vec![]; num_courses as usize];
        for edges in prerequisites.iter() {
            degrees[edges[0] as usize] += 1;
            descendants.get_mut(edges[1] as usize).unwrap_or(&mut vec![]).push(edges[0]);
        }
        let mut stack = Vec::new();
        for (i, &degree) in degrees.iter().enumerate() {
            if degree == 0 {
                stack.push(i);
            }
        }
        let mut result = Vec::new();
        while let Some(course) = stack.pop() {
            result.push(course as i32);
            let edges = &descendants[course];
            if edges.is_empty() {
                continue;
            }
            for edge in edges {
                degrees[*edge as usize] -= 1;
                if degrees[*edge as usize] == 0 {
                    stack.push(*edge as usize);
                }
            }
        }
        if result.len() == num_courses as usize {
            result
        } else {
            vec![]
        }
    }
}
struct Solution;
