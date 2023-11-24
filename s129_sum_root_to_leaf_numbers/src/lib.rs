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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(root) => Self::dfs(root, 0)
        }
    }

    fn dfs(root: Rc<RefCell<TreeNode>>, sum: i32) -> i32 {
        let left = root.borrow_mut().left.take();
        let right = root.borrow_mut().right.take();
        let sum = sum * 10 + root.borrow().val;
        if left.is_none() && right.is_none() {
            return sum;
        }
        left.into_iter().chain(right).map(|node| Self::dfs(node, sum)).sum()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let five = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let one = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let nine = Some(Rc::new(RefCell::new(TreeNode { val:   9,
                                                        left:  five,
                                                        right: one })));
        let zero = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let four = Some(Rc::new(RefCell::new(TreeNode { val:   4,
                                                        left:  nine,
                                                        right: zero })));
        assert_eq!(Solution::sum_numbers(four), 1026);
    }
}
