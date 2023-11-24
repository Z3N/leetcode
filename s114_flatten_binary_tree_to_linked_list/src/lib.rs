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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Self::dfs(root);
    }

    fn dfs(node: &mut Option<Rc<RefCell<TreeNode>>>) {
        if node.is_none() {
            return;
        }
        let mut stack = Vec::new();
        let dummy = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut last = dummy;
        stack.push(node.as_ref().unwrap().clone());
        while let Some(node) = stack.pop() {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            last.borrow_mut().right = Some(node.clone());
            last = node;
            if let Some(right) = right {
                stack.push(right);
            }
            if let Some(left) = left {
                stack.push(left);
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let three = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let four = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let two = Some(Rc::new(RefCell::new(TreeNode { val:   2,
                                                       left:  three,
                                                       right: four })));
        let six = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let five = Some(Rc::new(RefCell::new(TreeNode { val:   5,
                                                        left:  None,
                                                        right: six })));
        let one = Some(Rc::new(RefCell::new(TreeNode { val:   1,
                                                       left:  two,
                                                       right: five })));
        let mut root = one;
        let mut result = 0;
        Solution::flatten(&mut root);
        while let Some(node) = root {
            result += node.borrow().val;
            root = node.borrow_mut().right.take();
        }
        assert_eq!(result, 21);
    }
    #[test]
    fn it_works2() {
        let mut zero = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let result = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        Solution::flatten(&mut zero);
        assert_eq!(zero, result);
    }
}
