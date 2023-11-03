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
        let mut preorder = preorder.into_iter();
        let mut inorder = inorder.into_iter().peekable();
        let dummy = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let mut chain = dummy.clone();
        let mut traversal = Vec::new();
        traversal.push(TreeNode::new(preorder.next()?));
        while let Some(next_node) = traversal.pop() {
            if let Some(left_node) = inorder.next_if(|&x| x == next_node.val) {
                let mut root = traversal.pop()?;
                root.left = Some(Rc::new(RefCell::new(TreeNode::new(left_node))));
                inorder.next_if_eq(&root.val);
                if let Some(right_node) = inorder.next() {
                    root.right = Some(Rc::new(RefCell::new(TreeNode::new(right_node))));
                }
            } else {
                traversal.push(next_node);
            }
        }
        todo!()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let result = Solution::build_tree(preorder, inorder);
        assert_eq!(result, None);
    }
}
