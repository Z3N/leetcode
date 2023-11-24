use std::cell::RefCell;
use std::rc::Rc;

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

enum NodeDirection {
    Left(i32),
    Right(i32)
}

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
        let mut current = root.clone();
        let mut index = 0;
        let mut stack = Vec::new();
        for (i, &val) in preorder.iter().enumerate().skip(1) {
            if inorder[index] != preorder[i - 1] {
                let left_node = Rc::new(RefCell::new(TreeNode::new(val)));
                current.borrow_mut().left = Some(left_node.clone());
                stack.push(current.clone());
                current = left_node;
            } else {
                while !stack.is_empty() && stack.last()?.borrow().val == inorder[index + 1] {
                    current = stack.pop()?;
                    index += 1;
                }
                let right_node = Rc::new(RefCell::new(TreeNode::new(preorder[i])));
                current.borrow_mut().right = Some(right_node.clone());
                current = right_node;
                index += 1;
            }
        }
        Some(root)
    }
}

struct Solution;
