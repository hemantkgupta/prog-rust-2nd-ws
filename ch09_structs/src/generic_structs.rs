pub fn generic_structs_work(){

    let mut q = Queue::<String>::new1();
    let mut r = Queue::new2();

    q.push("CAD".to_string());  // apparently a Queue<&'static str>
    r.push(0.74);   // apparently a Queue<f64>

    q.push("BTC".to_string());   // Bitcoins per USD, 2019-6
    r.push(13764.0); // Rust fails to detect irrational exuberance
}

pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>
}

// Notice the <T> afte impl
impl<T> Queue<T> {

    pub fn new1() -> Queue<T> {
        Queue { older: Vec::new(), younger: Vec::new() }
    }

    pub fn new2() -> Self {
        Queue { older: Vec::new(), younger: Vec::new() }
    }

    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }


}

/* 
impl Queue<f64> {
    fn sum(&self) -> f64 {
        // ...
    }
}
*/

