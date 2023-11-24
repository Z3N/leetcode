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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }
        let mut queue = VecDeque::new();
        queue.push_front(root.unwrap());
        while !queue.is_empty() {
            let mut level = Vec::new();
            for _ in 0..queue.len() {
                let node = queue.pop_back().unwrap();
                if let Some(right) = node.borrow().right.clone() {
                    queue.push_front(right);
                };
                if let Some(left) = node.borrow().left.clone() {
                    queue.push_front(left);
                }
                level.push(node.borrow().val);
            }
            if result.len() % 2 != 0 {
                result.push(level)
            } else {
                result.push(level.into_iter().rev().collect())
            }
        }
        result
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let one = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = Solution::zigzag_level_order(one);
        assert_eq!(result, vec![vec![1]]);
    }
}
