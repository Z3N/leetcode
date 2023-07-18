use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut deque = vec![Vec::from_iter(root)];
        let mut result = Vec::new();
        while let Some(level) = deque.pop() {
            let element_count = level.len();
            let mut next_level = Vec::with_capacity(element_count * 2);
            let mut cumulative_value = 0.0;
            for node in level {
                let mut node = node.borrow_mut();
                cumulative_value += node.val as f64;
                if let Some(left) = node.left.take() {
                    next_level.push(left);
                }
                if let Some(right) = node.right.take() {
                    next_level.push(right);
                }
            }
            result.push(cumulative_value / element_count as f64);
            if next_level.is_empty() { break }
            deque.push(next_level);
        }
        result
    }
}

pub struct Solution;