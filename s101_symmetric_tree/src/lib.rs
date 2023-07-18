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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut deque = Vec::new();
        if let Some(root) = root {
            let mut root = root.borrow_mut();
            deque.push(root.left.take());
            deque.push(root.right.take());
        } else {
            return true;
        }
        while let (Some(left), Some(right)) = (deque.pop(), deque.pop()) {
            match (left, right) {
                (None, None) => continue,
                (Some(left), Some(right)) => {
                    let mut left = left.borrow_mut();
                    let mut right = right.borrow_mut();
                    if left.val != right.val { return false }
                    deque.push(left.right.take());
                    deque.push(right.left.take());
                    deque.push(left.left.take());
                    deque.push(right.right.take());
                }
                _ => return false
            }
        }
        true
    }
}

pub struct Solution;