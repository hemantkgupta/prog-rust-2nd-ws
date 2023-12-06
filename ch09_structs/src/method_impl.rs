pub fn method_impl_work(){
    let mut q = Queue {
        older : Vec::new(),
        younger : Vec::new()
    };

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('∞');
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), None);

    assert_eq!(q.is_empty(), true);
    assert!(q.is_empty());
    q.push('☉');
    assert!(!q.is_empty());

    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');

    let (older, younger) = q.split();
    // q is now uninitialized.
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);

    let mut bq = Box::new(Queue::new());

    // `Queue::push` expects a `&mut Queue`, but `bq` is a `Box<Queue>`.
    // This is fine: Rust borrows a `&mut Queue` from the `Box` for the
    // duration of the call.
    bq.push('■');

    let mut q2 = Queue::new();

    q2.push('*');


}

pub struct Queue{
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue{

    pub fn new() -> Queue {
        Queue { older: Vec::new(), younger: Vec::new() }
    }

    pub fn push(&mut self, c:char){
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<char>{
        if self.older.is_empty(){
            if self.younger.is_empty(){
                return None;
            }
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}