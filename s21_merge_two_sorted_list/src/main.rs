fn main() {
    println!("Hello, world!");
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() || list2.is_none() {
            return list1.or(list2);
        }
        let mut smaller = list1.unwrap();
        let mut greater = list2.unwrap();
        if smaller.val > greater.val {
            std::mem::swap(&mut smaller,&mut greater);
        }
        let head = smaller.clone();
        while let (Some(mut first_node), Some(second_node)) = (smaller.next, greater.next) {


            todo!()
        }

        Some(head)
    }
    fn connect_nodes(first: Option<Box<ListNode>>, second: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut first_node) = first {
            first_node.next = second;
            Some(first_node)
        } else {
            second
        }
    }
}

struct Solution;