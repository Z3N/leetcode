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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Self::dfs(root, k as u32, 0).unwrap()
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, k: u32, depth: u32) -> Result<i32, u32> {
        if let Some(root) = root {
            let depth = match Self::dfs(root.borrow_mut().left.take(), k, depth) {
                Ok(success) => return Ok(success),
                Err(depth) => depth
            };
            let depth = depth + 1;
            if depth == k {
                return Ok(root.borrow().val);
            }
            Self::dfs(root.borrow_mut().right.take(), k, depth)
        } else {
            Err(depth)
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let four = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let two = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let one = Some(Rc::new(RefCell::new(TreeNode { left:  None,
                                                       right: two,
                                                       val:   1 })));
        let three = Some(Rc::new(RefCell::new(TreeNode { left:  one,
                                                         right: four,
                                                         val:   3 })));
        let result = Solution::kth_smallest(three, 1);
        assert_eq!(result, 1);
    }

    #[test]
    fn it_does_not_works() {
        let one = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let two = Some(Rc::new(RefCell::new(TreeNode { left:  one,
                                                       right: None,
                                                       val:   2 })));
        let four = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let three = Some(Rc::new(RefCell::new(TreeNode { left:  two,
                                                         right: four,
                                                         val:   3 })));
        let six = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let five = Some(Rc::new(RefCell::new(TreeNode { left:  three,
                                                        right: six,
                                                        val:   5 })));
        let result = Solution::kth_smallest(five, 3);
        assert_eq!(result, 3);
    }
    #[test]
    fn pray_it_works() {
        let two = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let one = Some(Rc::new(RefCell::new(TreeNode { left:  None,
                                                       right: two,
                                                       val:   1 })));
        let result = Solution::kth_smallest(one, 2);
        assert_eq!(result, 2);
    }
}
