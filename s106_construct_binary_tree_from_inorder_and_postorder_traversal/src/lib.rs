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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let root = Rc::new(RefCell::new(TreeNode::new(postorder[postorder.len() - 1])));
        let mut current = root.clone();
        let mut index = postorder.len() - 1;
        let mut stack = Vec::new();
        for (i, &val) in postorder.iter().enumerate().rev().skip(1) {
            if inorder[index] != postorder[i + 1] {
                let right_node = Rc::new(RefCell::new(TreeNode::new(val)));
                current.borrow_mut().right = Some(right_node.clone());
                stack.push(current.clone());
                current = right_node;
            } else {
                while !stack.is_empty() && stack.last()?.borrow().val == inorder[index - 1] {
                    current = stack.pop()?;
                    index -= 1;
                }
                let left_node = Rc::new(RefCell::new(TreeNode::new(postorder[i])));
                current.borrow_mut().left = Some(left_node.clone());
                current = left_node;
                index -= 1;
            }
        }
        Some(root)
    }
}

struct Solution;

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
