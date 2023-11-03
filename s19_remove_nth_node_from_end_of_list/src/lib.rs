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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let n = n as usize;
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut fast = dummy.clone();
        let mut fast = fast.as_mut();
        let mut slow = dummy.as_mut();
        for _ in 0..n {
            fast = fast?.next.as_mut();
        }
        while fast.as_mut()?.next.is_some() {
            slow = slow?.next.as_mut();
            fast = fast?.next.as_mut();
        }
        let tail = slow.as_mut()?.next.take();
        slow.as_mut()?.next = tail?.next.take();
        dummy?.next.take()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let four = Some(Box::new(ListNode { val:  4,
                                            next: Some(Box::new(ListNode::new(5))) }));
        let three = Some(Box::new(ListNode { val: 3, next: four }));
        let two = Some(Box::new(ListNode { val: 2, next: three }));
        let one = Some(Box::new(ListNode { val: 1, next: two }));

        let cut_five = Some(Box::new(ListNode { val: 5, next: None }));
        let cut_two = Some(Box::new(ListNode { val:  2,
                                               next: Some(Box::new(ListNode { val: 3, next: cut_five })) }));
        let cut_one = Some(Box::new(ListNode { val: 1, next: cut_two }));
        let result = Solution::remove_nth_from_end(one, 2);
        assert_eq!(cut_one, result);
    }
}
