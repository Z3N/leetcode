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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = Vec::new();
        let mut root = root;
        let mut prev = None;
        while root.is_some() || !stack.is_empty() {
            while let Some(node) = root {
                root = node.borrow().left.clone();
                stack.push(node);
            }
            let node = stack.pop().unwrap();
            if let Some(value) = prev {
                if node.borrow().val <= value {
                    return false;
                }
            }
            prev = Some(node.borrow().val);
            root = node.borrow_mut().right.take();
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let one = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let three = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let two = Some(Rc::new(RefCell::new(TreeNode { left:  one,
                                                       right: three,
                                                       val:   2 })));
        let result = Solution::is_valid_bst(two);

        assert_eq!(result, true);
    }
}
