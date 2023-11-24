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
struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self { stack: vec![root.unwrap()] }
    }

    fn next(&mut self) -> i32 {
        self.unwind().borrow().val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }

    fn unwind(&mut self) -> Rc<RefCell<TreeNode>> {
        let mut node = self.stack.pop().unwrap();
        while node.borrow().right.is_some() || node.borrow().left.is_some() {
            let right = node.borrow_mut().right.take();
            let left = node.borrow_mut().left.take();
            if let Some(right) = right {
                self.stack.push(right);
            }
            if let Some(left) = left {
                self.stack.push(node);
                node = left;
            } else {
                break;
            }
        }
        node
    }
}
