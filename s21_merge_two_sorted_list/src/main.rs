// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>, ) -> Option<Box<ListNode>> {
        let mut body = &mut list1;
        while list2.is_some() {
            if body.is_none() || body.as_ref()?.val > list2.as_ref()?.val {
                std::mem::swap(body, &mut list2);
            }
            body = &mut body.as_mut()?.next;
        }
        list1
    }
}

struct Solution;
