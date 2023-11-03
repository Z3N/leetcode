// Definition for singly-linked list.
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
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut fake_head = Some(Box::new(ListNode { val: 0, next: head }));
        let head = fake_head.as_mut();
        if left >= right {
            return fake_head?.next;
        }
        let mut body = head;
        for _ in 1..left {
            body = body?.next.as_mut();
        }
        let raw_tail = Box::into_raw(body.as_mut()?.next.take()?);
        let mut fuse = unsafe { Some(Box::from_raw(raw_tail)) };
        let mut tail = fuse.as_mut()?.next.take();
        body.as_mut()?.next = fuse;
        for _ in left..right {
            let lhs = body.as_mut()?.next.take();
            let mut rhs = tail;
            let next_tail = rhs.as_mut()?.next.take();
            rhs.as_mut()?.next = lhs;
            body.as_mut()?.next = rhs;
            tail = next_tail;
        }
        unsafe { (*raw_tail).next = tail }
        fake_head?.next
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let five = Box::new(ListNode::new(5));
        let mut four = Box::new(ListNode::new(4));
        let mut three = Box::new(ListNode::new(3));
        let mut two = Box::new(ListNode::new(2));
        let mut one = Box::new(ListNode::new(1));
        four.next = Some(five);
        three.next = Some(four);
        two.next = Some(three);
        one.next = Some(two);
        let mut r_four = Box::new(ListNode::new(4));
        let mut r_three = Box::new(ListNode::new(3));
        let mut r_two = Box::new(ListNode::new(2));
        let mut r_one = Box::new(ListNode::new(1));
        let r_five = Box::new(ListNode::new(5));
        r_two.next = Some(r_five);
        r_three.next = Some(r_two);
        r_four.next = Some(r_three);
        r_one.next = Some(r_four);
        let result = Solution::reverse_between(Some(one), 2, 4);
        assert_eq!(result, Some(r_one));
    }

    #[test]
    fn it_works2() {
        let five = Box::new(ListNode::new(5));
        let mut three = Box::new(ListNode::new(3));
        three.next = Some(five);
        let mut r_five = Box::new(ListNode::new(5));
        let r_three = Box::new(ListNode::new(3));
        r_five.next = Some(r_three);
        let result = Solution::reverse_between(Some(three), 1, 2);
        assert_eq!(result, Some(r_five));
    }

    #[test]
    fn it_works3() {
        let three = Box::new(ListNode::new(3));
        let mut r_three = Box::new(ListNode::new(3));
        let mut two = Box::new(ListNode::new(2));
        let mut r_two = Box::new(ListNode::new(2));
        let mut one = Box::new(ListNode::new(1));
        let r_one = Box::new(ListNode::new(1));
        two.next = Some(three);
        one.next = Some(two);
        r_two.next = Some(r_one);
        r_three.next = Some(r_two);
        let result = Solution::reverse_between(Some(one), 1, 3);
        assert_eq!(result, Some(r_three));
    }
}
