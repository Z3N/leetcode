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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 {
            return head;
        }
        let mut chain = head.as_mut();
        let mut len = 0;
        while let Some(node) = chain {
            chain = node.next.as_mut();
            len += 1;
        }
        if len == 0 {
            return head;
        }
        let mut chain = head.as_mut();
        let shift = k as usize % len;
        if shift == 0 {
            return head;
        }
        for _ in 0..len - shift - 1 {
            chain = chain?.next.as_mut();
        }
        let mut new_head = chain?.next.take();
        let mut new_chain = new_head.as_mut();
        while let Some(node) = new_chain {
            if node.next.is_some() {
                new_chain = node.next.as_mut();
            } else {
                node.next = head;
                break;
            }
        }
        new_head
    }

    pub fn rotate_right_vec(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 {
            return head;
        }
        let mut intermediate = Vec::new();
        let mut chain = head.as_mut();
        while let Some(node) = chain {
            intermediate.push(node.val);
            chain = node.next.as_mut();
        }
        if intermediate.is_empty() {
            return None;
        }
        let k = k as usize % intermediate.len();
        let head_index = (intermediate.len() - k) % intermediate.len();
        let mut chain = head.as_mut();
        for &value in intermediate.iter().skip(head_index) {
            chain.as_mut()?.val = value;
            chain = chain?.next.as_mut();
        }
        for value in intermediate.into_iter().take(head_index) {
            chain.as_mut()?.val = value;
            chain = chain?.next.as_mut();
        }
        head
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let five = Some(Box::new(ListNode::new(5)));
        let four = Some(Box::new(ListNode { val: 4, next: five }));
        let three = Some(Box::new(ListNode { val: 3, next: four }));
        let two = Some(Box::new(ListNode { val: 2, next: three }));
        let one = Some(Box::new(ListNode { val: 1, next: two }));

        let r_five = Some(Box::new(ListNode::new(3)));
        let r_four = Some(Box::new(ListNode { val: 2, next: r_five }));
        let r_three = Some(Box::new(ListNode { val: 1, next: r_four }));
        let r_two = Some(Box::new(ListNode { val: 5, next: r_three }));
        let r_one = Some(Box::new(ListNode { val: 4, next: r_two }));

        let result = Solution::rotate_right(one, 2);
        assert_eq!(result, r_one);
    }

    #[test]
    fn it_doesnt_work() {
        let two = Some(Box::new(ListNode { val: 2, next: None }));
        let one = Some(Box::new(ListNode { val: 1, next: two }));
        let r_two = Some(Box::new(ListNode { val: 1, next: None }));
        let r_one = Some(Box::new(ListNode { val: 2, next: r_two }));
        let result = Solution::rotate_right(one, 1);
        assert_eq!(result, r_one);
    }

    #[test]
    fn it_works_three() {
        let three = Some(Box::new(ListNode { val: 3, next: None }));
        let two = Some(Box::new(ListNode { val: 2, next: three }));
        let one = Some(Box::new(ListNode { val: 1, next: two }));
        let r_three = Some(Box::new(ListNode { val: 1, next: None }));
        let r_two = Some(Box::new(ListNode { val: 3, next: r_three }));
        let r_one = Some(Box::new(ListNode { val: 2, next: r_two }));
        let result = Solution::rotate_right(one, 2000000000);
        assert_eq!(result, r_one);
    }
}
