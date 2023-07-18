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

trait NodeSwap {
    fn mirror_swap(&mut self);
}

impl NodeSwap for TreeNode {
    fn mirror_swap(&mut self) {
        if let Some(right) = self.right.as_mut() { right.borrow_mut().mirror_swap() }
        if let Some(left) = self.left.as_mut() { left.borrow_mut().mirror_swap() }
        std::mem::swap(&mut self.left, &mut self.right);
    }
}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.map(|root| {
            root.borrow_mut().mirror_swap();
            root
        })
    }
}

pub struct Solution;