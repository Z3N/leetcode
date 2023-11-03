use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct LRUCache {
    capacity: usize,
    map:      HashMap<i32, Rc<RefCell<Box<Node>>>>,
    list:     Option<Rc<RefCell<Box<Node>>>>,
    tail:     Option<Rc<RefCell<Box<Node>>>>,
    size:     usize
}

#[derive(PartialEq)]
struct Node {
    key:   i32,
    value: i32,
    prev:  Option<Rc<RefCell<Box<Node>>>>,
    next:  Option<Rc<RefCell<Box<Node>>>>
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache { capacity: capacity as usize,
                   map:      HashMap::with_capacity(capacity as usize),
                   list:     None,
                   tail:     None,
                   size:     0 }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key) {
            Some(node) => {
                let value = node.borrow().value;
                self.update_last_used(node.clone());
                value
            }
            None => -1
        }
    }

    fn update_last_used(&mut self, node: Rc<RefCell<Box<Node>>>) {
        if let Some(tail) = self.tail.as_mut() {
            if tail.borrow().key == node.borrow().key {
                return;
            }
        }
        let mut next = node.borrow_mut().next.take();
        let prev = node.borrow_mut().prev.clone();
        if let Some(previous) = prev.clone() {
            previous.borrow_mut().next = next.clone();
            next.as_mut().unwrap().borrow_mut().prev = prev;
        } else {
            next.as_mut().and_then(|x| x.borrow_mut().prev.take());
            self.list = next.clone().or_else(|| Some(node.clone()));
        }
        // add next and prev check
        self.tail.as_mut().map(|x| x.borrow_mut().next = Some(node.clone()));
        node.borrow_mut().prev = self.tail.clone();
        node.borrow_mut().next.take();
        self.tail = Some(node.clone());
        if let Some(node) = self.tail.as_ref().map(|x| x.borrow().next.clone()) {
            if node.is_some() {
                eprintln!("ERROR in update_last_used: {:?}", node.unwrap().borrow().key);
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key) {
            node.borrow_mut().value = value;
            self.update_last_used(node.clone());
        } else {
            if self.size < self.capacity {
                self.size += 1;
            } else {
                let remove = self.list.take().unwrap();
                self.list = remove.borrow_mut().next.take();
                self.list.as_mut().and_then(|x| x.borrow_mut().prev.take());
                self.map.remove(&remove.borrow().key);
            }
            let node = Rc::new(RefCell::new(Box::new(Node { key,
                                                            value,
                                                            prev: self.tail.clone(),
                                                            next: None })));
            if self.list.is_none() {
                self.list = Some(node.clone());
            }
            if let Some(tail) = self.tail.as_mut() {
                tail.borrow_mut().next = Some(node.clone());
            }
            self.tail = Some(node.clone());
            self.map.insert(key, node);
            if let Some(node) = self.tail.as_ref().map(|x| x.borrow().next.clone()) {
                if node.is_some() {
                    eprintln!("ERROR in put: {:?}", node.unwrap().borrow().key);
                }
            }
        };
    }

    fn struct_valker(&mut self) {
        let mut result = String::new();
        let head_prev_is_clean = self.list.as_mut().map(|x| x.borrow_mut().prev.is_none());
        let tail_next_is_clean = self.tail.as_mut().map(|x| x.borrow_mut().next.is_none());
        result += &format!("List and tail are clean: {head_prev_is_clean:?} {tail_next_is_clean:?}\n");
        let mut list_length = 0;
        let mut chain = self.list.clone();
        while let Some(node) = chain {
            result += &format!("{:?}<--| ({:?},{:?}) |-->{:?} ---",
                               node.borrow().prev.as_ref().map(|x| x.borrow().key),
                               node.borrow().key,
                               node.borrow().value,
                               node.borrow().next.as_ref().map(|x| x.borrow().key));
            chain = node.borrow().next.clone();
            list_length += 1;
        }
        let size_check = self.map.len() == list_length;
        result += &format!("\nLength of map and list is equal: {size_check}\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 0;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let mut result = LRUCache::new(2);
        result.put(1, 1);
        result.put(2, 2);
        assert_eq!(result.get(1), 1);
        result.put(3, 3);
        assert_eq!(result.get(2), -1);
        result.put(4, 4);
        assert_eq!(result.get(1), -1);
        assert_eq!(result.get(3), 3);
        assert_eq!(result.get(4), 4);
    }

    #[test]
    fn it_doesnt_work() {
        let mut result = LRUCache::new(10);
        let feed_data = vec![vec![10, 13],
                             vec![3, 17],
                             vec![6, 11],
                             vec![10, 5],
                             vec![9, 10],
                             vec![13],
                             vec![2, 19],
                             vec![2],
                             vec![3],
                             vec![5, 25],
                             vec![8],
                             vec![9, 22],
                             vec![5, 5],
                             vec![1, 30],
                             vec![11],
                             vec![9, 12],
                             vec![7],
                             vec![5],
                             vec![8],
                             vec![9],
                             vec![4, 30],
                             vec![9, 3],
                             vec![9],
                             vec![10],
                             vec![10],
                             vec![6, 14],
                             vec![3, 1],
                             vec![3],
                             vec![10, 11],
                             vec![8],
                             vec![2, 14],
                             vec![1],
                             vec![5],
                             vec![4],
                             vec![11, 4],
                             vec![12, 24],
                             vec![5, 18],
                             vec![13],
                             vec![7, 23],
                             vec![8],
                             vec![12],
                             vec![3, 27],
                             vec![2, 12],
                             vec![5],
                             vec![2, 9],
                             vec![13, 4],
                             vec![8, 18],
                             vec![1, 7],
                             vec![6],
                             vec![9, 29],
                             vec![8, 21],
                             vec![5],
                             vec![6, 30],
                             vec![1, 12],
                             vec![10],
                             vec![4, 15],
                             vec![7, 22],
                             vec![11, 26],
                             vec![8, 17],
                             vec![9, 29],
                             vec![5],
                             vec![3, 4],
                             vec![11, 30],
                             vec![12],
                             vec![4, 29],
                             vec![3],
                             vec![9],
                             vec![6],
                             vec![3, 4],
                             vec![1],
                             vec![10],
                             vec![3, 29],
                             vec![10, 28],
                             vec![1, 20],
                             vec![11, 13],
                             vec![3],
                             vec![3, 12],
                             vec![3, 8],
                             vec![10, 9],
                             vec![3, 26],
                             vec![8],
                             vec![7],
                             vec![5],
                             vec![13, 17],
                             vec![2, 27],
                             vec![11, 15],
                             vec![12],
                             vec![9, 19],
                             vec![2, 15],
                             vec![3, 16],
                             vec![1],
                             vec![12, 17],
                             vec![9, 1],
                             vec![6, 19],
                             vec![4],
                             vec![5],
                             vec![5],
                             vec![8, 1],
                             vec![11, 7],
                             vec![5, 2],
                             vec![9, 28],
                             vec![1],
                             vec![2, 2],
                             vec![7, 4],
                             vec![4, 22],
                             vec![7, 24],
                             vec![9, 26],
                             vec![13, 28],
                             vec![11, 26]];
        let expected = vec![None,
                            None,
                            None,
                            None,
                            None,
                            Some(-1),
                            None,
                            Some(19),
                            Some(17),
                            None,
                            Some(-1),
                            None,
                            None,
                            None,
                            Some(-1),
                            None,
                            Some(-1),
                            Some(5),
                            Some(-1),
                            Some(12),
                            None,
                            None,
                            Some(3),
                            Some(5),
                            Some(5),
                            None,
                            None,
                            Some(1),
                            None,
                            Some(-1),
                            None,
                            Some(30),
                            Some(5),
                            Some(30),
                            None,
                            None,
                            None,
                            Some(-1),
                            None,
                            Some(-1),
                            Some(24),
                            None,
                            None,
                            Some(18),
                            None,
                            None,
                            None,
                            None,
                            Some(-1),
                            None,
                            None,
                            Some(18),
                            None,
                            None,
                            Some(-1),
                            None,
                            None,
                            None,
                            None,
                            None,
                            Some(18),
                            None,
                            None,
                            Some(-1),
                            None,
                            Some(4),
                            Some(29),
                            Some(30),
                            None,
                            Some(12),
                            Some(-1),
                            None,
                            None,
                            None,
                            None,
                            Some(29),
                            None,
                            None,
                            None,
                            None,
                            Some(17),
                            Some(22),
                            Some(18),
                            None,
                            None,
                            None,
                            Some(-1),
                            None,
                            None,
                            None,
                            Some(20),
                            None,
                            None,
                            None,
                            Some(-1),
                            Some(18),
                            Some(18),
                            None,
                            None,
                            None,
                            None,
                            Some(20),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None];
        assert_eq!(feed_data.len(), expected.len());
        for (data, expected) in feed_data.into_iter().zip(expected) {
            match data.len() == 2 {
                true => {
                    result.put(data[0], data[1]);
                }
                false => {
                    let before = result.struct_valker();
                    let current = result.get(data[0]);
                    if Some(current) != expected {
                        let after = result.struct_valker();
                        eprintln!("Before: {before:?}\nAfter: {after:?}\n");
                        assert_eq!(current, expected.unwrap());
                        break;
                    }
                }
            }
        }
    }
}
