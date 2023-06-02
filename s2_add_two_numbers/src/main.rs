fn main() {
    println!("Hello, world!");
}

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

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carrying = 0;
        let mut result_head = Box::new(ListNode::new(0));
        let mut result_next = &mut result_head;
        let mut first_node = l1;
        let mut second_node = l2;
        while first_node.is_some() || second_node.is_some() || carrying != 0 {
            carrying += first_node
                .iter()
                .chain(second_node.iter())
                .map(|x| x.val)
                .sum::<i32>();
            result_next.next = Some(Box::new(ListNode::new(carrying % 10)));
            result_next = result_next.next.as_mut().unwrap();
            carrying /= 10;

            first_node = if let Some(node) = first_node {
                node.next
            } else {
                None
            };
            second_node = if let Some(node) = second_node {
                node.next
            } else {
                None
            };
        }
        result_head.next
    }
}
