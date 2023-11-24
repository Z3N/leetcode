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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }
        let mut stack = Vec::new();
        stack.push((root.unwrap(), 0));
        while let Some((node, level)) = stack.pop() {
            if level >= result.len() {
                result.push(Vec::new());
            }
            result[level].push(node.borrow().val);
            if let Some(right) = node.borrow_mut().right.take() {
                stack.push((right, level + 1));
            }
            if let Some(left) = node.borrow_mut().left.take() {
                stack.push((left, level + 1));
            }
        }
        result
    }
}
struct Solution;
