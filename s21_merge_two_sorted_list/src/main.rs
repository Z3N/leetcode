fn main() {
    println!("Hello, world!");
    test_move(vec![34])
}
fn test_move(vec: Vec<u32>) {
    println!()
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
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() || list2.is_none() {
            return list1.or(list2);
        }
        let (mut head, mut second) = match list1.as_ref()?.val > list2.as_ref()?.val {
            true => (list1, list2),
            false => (list2, list1),
        };
        /*        let mut first = head.next.take();
        if first.is_none() {
            std::mem::swap(&mut head.next, &mut Some(second));
            return Some(head);
        }
        let mut first = first.unwrap();

        if first.val > second.val {
            let prev = first.next.take();
            head.next = Some(first);
            first = first.next;
        }

        if first.val > second.val { }*/
        /*let mut smaller = list1.unwrap();
        let mut bigger = list2.unwrap();
        if smaller.val > bigger.val {
            std::mem::swap(&mut smaller, &mut bigger);
        }
        let head = smaller.clone();*/

        todo!()
    }
    fn connect_nodes(
        first: Option<Box<ListNode>>,
        second: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(mut first_node) = first {
            first_node.next = second;
            Some(first_node)
        } else {
            second
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yourself() {
        let result = Solution::merge_two_lists(None, None);
        assert_eq!(result, None);
    }
}
