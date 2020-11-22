pub struct MinStack<T> {
    stack: Vec<T>,    
    min_stack: Vec<usize>
}

impl<T : Ord> MinStack<T> {    
    #[inline]
    pub fn new() -> MinStack<T> {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new()
        }
    }
    #[inline]
    pub fn push(&mut self, value: T) {
        let idx = self.stack.len();

        match self.min_stack.last() {
            Some(&min) if value <= self.stack[min] =>
                self.min_stack.push(idx),
            None =>
                self.min_stack.push(idx),
            _ => { }
        }

        self.stack.push(value);
    }    
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        let value = self.stack.pop();

        match self.min_stack.last() {
            Some(&min) if min == self.stack.len() => {
                self.min_stack.pop();
            }
            _ => { }
        }

        value
    }
    #[inline]
    pub fn min(&self) -> Option<&T> {
        self.min_stack.last().map(|&n| &self.stack[n])
    }
}


fn main(){
    let mut stack : MinStack<i32> = MinStack::new();
    stack.push(3);
    stack.push(2);
    stack.push(1);
    
    print!("{:?}", stack.min());
}