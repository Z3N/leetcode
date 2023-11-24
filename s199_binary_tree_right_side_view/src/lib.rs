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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vec = Vec::new();
        if root.is_none() {
            return vec;
        }
        let mut stack = Vec::new();
        stack.push((root.unwrap(), 0));
        while let Some((current_node, current_depth)) = stack.pop() {
            let left = current_node.borrow_mut().left.take();
            if let Some(left) = left {
                stack.push((left, current_depth + 1));
            }
            let right = current_node.borrow_mut().right.take();
            if let Some(right) = right {
                stack.push((right, current_depth + 1));
            }
            if current_depth == vec.len() {
                vec.push(current_node.borrow().val);
            }
        }
        vec
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let three = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let one = Some(Rc::new(RefCell::new(TreeNode { val:   1,
                                                       left:  None,
                                                       right: three })));
        let result = Solution::right_side_view(one);
        assert_eq!(result, vec![1, 3]);
    }
}
