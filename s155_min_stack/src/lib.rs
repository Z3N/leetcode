struct MinStack {
    data: Vec<i32>,
    min:  Vec<i32>
}

impl MinStack {
    fn new() -> Self {
        MinStack { data: Vec::new(),
                   min:  vec![] }
    }

    fn push(&mut self, val: i32) {
        self.data.push(val);
        if self.min.is_empty() || self.min.last() >= Some(&val) {
            self.min.push(val);
        }
    }

    fn pop(&mut self) {
        if self.data.last() == self.min.last() {
            self.min.pop();
        }
        self.data.pop();
    }

    fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}
