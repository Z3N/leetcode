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
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut new_tail = None;
        let mut tail_chain = &mut new_tail;
        let mut new_head = None;
        let mut head_chain = &mut new_head;
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val >= x {
                tail_chain = &mut tail_chain.insert(node).next;
            } else {
                head_chain = &mut head_chain.insert(node).next;
            }
        }
        if let Some(tail) = new_tail {
            let _ = head_chain.insert(tail);
        }
        new_head
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expected = Some(Box::new(ListNode::new(1)));
        let one = Some(Box::new(ListNode::new(1)));
        let result = Solution::partition(one, 2);
        assert_eq!(result, expected);
    }
}
