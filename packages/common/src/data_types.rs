pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn is_empty(&self) -> bool {
        self.stack.len() == 0
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

impl Drop for Stack<T> {
    fn default() -> Self {
        Stack { stack: Vec::new() }
    }
}
