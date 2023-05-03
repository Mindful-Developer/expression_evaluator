pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            stack: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    pub fn last(&self) -> Option<&T> {
        self.stack.last()
    }
}
