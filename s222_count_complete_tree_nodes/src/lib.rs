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

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::count_by_ref(&root)
    }

    pub fn count_by_ref(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let r_height = Self::height_right(root);
        let l_height = Self::height_left(root);
        match root {
            None => 0,
            Some(root) => {
                if r_height == l_height {
                    2_i32.pow(r_height as u32) - 1
                } else {
                    1
                    + Self::count_by_ref(&root.borrow().left)
                    + Self::count_by_ref(&root.borrow().right)
                }
            }
        }
    }

    fn height_right(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
        match root {
            None => 0,
            Some(right) => Self::height_right(&right.borrow().right) + 1
        }
    }

    fn height_left(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
        match root {
            None => 0,
            Some(left) => Self::height_left(&left.borrow().left) + 1
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tree = Rc::new(RefCell::new(TreeNode::new(1)));
        let two = Rc::new(RefCell::new(TreeNode::new(2)));
        two.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        two.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let three = Rc::new(RefCell::new(TreeNode::new(3)));
        three.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        tree.borrow_mut().right = Some(three);
        tree.borrow_mut().left = Some(two);
        let result = Solution::count_nodes(Some(tree));
        assert_eq!(result, 6);
    }
    #[test]
    fn eight() {
        let tree = Rc::new(RefCell::new(TreeNode::new(1)));
        let two = Rc::new(RefCell::new(TreeNode::new(2)));
        let four = Rc::new(RefCell::new(TreeNode::new(4)));
        four.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        two.borrow_mut().left = Some(four);
        two.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let three = Rc::new(RefCell::new(TreeNode::new(3)));
        three.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        three.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        tree.borrow_mut().right = Some(three);
        tree.borrow_mut().left = Some(two);
        let result = Solution::count_nodes(Some(tree));
        assert_eq!(result, 8);
    }
}
