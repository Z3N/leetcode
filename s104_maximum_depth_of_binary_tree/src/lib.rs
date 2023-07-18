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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::tree_walker(root)
    }
    pub fn tree_walker(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let root = Rc::try_unwrap(root).unwrap().into_inner();
            Solution::tree_walker(root.left).max(Solution::tree_walker(root.right)) + 1
        } else {
            0
        }
    }
    pub fn tree_walker_clone(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            Solution::tree_walker(root.borrow().left.clone()).max(Solution::tree_walker(root.borrow().right.clone())) + 1
        } else {
            0
        }
    }
    pub fn tree_walker_move(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let mut root = root.borrow_mut();
            Solution::tree_walker(root.left.take()).max(Solution::tree_walker(root.right.take())) + 1
        } else {
            0
        }
    }
}

pub struct Solution;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
