#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val:  i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut new_head = &mut result;
        while let Some(mut node) = head {
            if node.next.is_none() || node.val != node.next.as_ref()?.val {
                head = node.next.take();
                new_head = &mut new_head.insert(node).next;
            } else {
                head = node.next?.next.take();
                while head.is_some() && head.as_ref()?.val == node.val {
                    head = head.as_mut()?.next.take();
                }
            }
        }
        result
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
        let five = Some(Box::new(ListNode::new(3)));
        let four = Some(Box::new(ListNode { val: 2, next: five }));
        let three = Some(Box::new(ListNode { val: 1, next: four }));
        let two = Some(Box::new(ListNode { val: 1, next: three }));
        let one = Some(Box::new(ListNode { val: 1, next: two }));
        let should_be = Some(Box::new(ListNode { val:  2,
                                                 next: Some(Box::new(ListNode { val: 3, next: None })) }));
        let result = Solution::delete_duplicates(one);
        assert_eq!(result, should_be);
    }

    #[test]
    fn it_doesnt_work() {
        let five = Some(Box::new(ListNode::new(5)));
        let four = Some(Box::new(ListNode { val: 4, next: five }));
        let three = Some(Box::new(ListNode { val: 4, next: four }));
        let two = Some(Box::new(ListNode { val: 1, next: three }));
        let one = Some(Box::new(ListNode { val: 1, next: two }));
        let should_be = Some(Box::new(ListNode::new(5)));
        let result = Solution::delete_duplicates(one);
        assert_eq!(result, should_be);
    }

    #[test]
    fn test_yourself() {
        let one = Some(Box::new(ListNode { val:  1,
                                           next: Some(Box::new(ListNode::new(1))) }));
        let result = Solution::delete_duplicates(one);
        assert_eq!(result, None);
    }

    #[test]
    fn test_repetition() {
        let three = Some(Box::new(ListNode { val: 1, next: None }));
        let two = Some(Box::new(ListNode { val: 1, next: three }));
        let one = Some(Box::new(ListNode { val: 1, next: two }));
        let result = Solution::delete_duplicates(one);
        assert_eq!(result, None);
    }
}
