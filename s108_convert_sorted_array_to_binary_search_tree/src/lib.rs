use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree(&nums)
    }
    fn build_tree(elements: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if elements.is_empty() { return None; }
        let (left, rest) = elements.split_at(elements.len() / 2);
        let (mid, right) = rest.split_first()?;
        Some(
            Rc::new(
                RefCell::new(TreeNode {
                    val: *mid,
                    left: Self::build_tree(left),
                    right: Self::build_tree(right)
                })
            )
        )
    }
}

pub struct Solution;

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
