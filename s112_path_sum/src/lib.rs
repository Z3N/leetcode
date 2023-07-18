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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match (root, target_sum) {
            (Some(root), _) => {
                let mut root = root.borrow_mut();
                target_sum == root.val && root.left.is_none() && root.right.is_none()
                    ||
                    Self::has_path_sum(root.left.take(), target_sum - root.val) ||
                    Self::has_path_sum(root.right.take(), target_sum - root.val)
            }
            _ => false
        }
    }
}

pub struct Solution;
