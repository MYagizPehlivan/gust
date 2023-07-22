pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, element: T) {
        self.data.push(element);
    }

    pub fn pop(&mut self) -> T {
        self.data.pop().unwrap()
    }
}
