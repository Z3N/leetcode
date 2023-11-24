#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val:   i32,
    pub left:  Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val,
                   left: None,
                   right: None }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>,
                                  p: Option<Rc<RefCell<TreeNode>>>,
                                  q: Option<Rc<RefCell<TreeNode>>>)
                                  -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            if root.borrow().val == p.clone()?.borrow().val || root.borrow().val == q.clone()?.borrow().val {
                return Some(root);
            }
            let left = Self::lowest_common_ancestor(root.borrow().left.clone(), p.clone(), q.clone());
            let right = Self::lowest_common_ancestor(root.borrow().right.clone(), p.clone(), q.clone());
            if left.is_some() && right.is_some() {
                return Some(root);
            }
            if left.is_some() {
                return left;
            }
            right
        } else {
            None
        }
    }
}
struct Solution;
