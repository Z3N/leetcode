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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut prev_node = None;
        let mut min = i32::MAX;
        Self::traverse(root, &mut min, &mut prev_node);
        min
    }
    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, min: &mut i32, prev_value: &mut Option<i32>) {
        if let Some(root) = root {
            let mut root = root.borrow_mut();
            Self::traverse(root.left.take(), min, prev_value);
            if let Some(smaller) = *prev_value {
                *min = std::cmp::min(*min, root.val - smaller);
            }
            *prev_value = Some(root.val);
            Self::traverse(root.right.take(), min, prev_value);
        }
    }
}

pub struct Solution;