pub struct Stack<T> {
    stack: Vec<T>,
    max_size: usize,
}

impl<T> Stack<T> {
    pub fn new(max_size: usize) -> Stack<T> {
        Stack {
            stack: Vec::with_capacity(max_size),
            max_size,
        }
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn push(&mut self, item: T) {
        if self.len() != self.max_size {
            self.stack.push(item);
        } else {
            panic!("Push to full stack")
        }
    }

    pub fn last(&self) -> Option<&T> {
        self.stack.last()
    }
}
