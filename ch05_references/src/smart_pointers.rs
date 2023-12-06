pub fn smart_pointers_work(){
    
    // Like a Wrapper in Java
    let b = Box::new(5);
    println!("b = {}", b);

    // Using Box<T> to Get a Recursive Type with a Known Size
    //let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    //println!("The list is : {:?}", list)

    // Following the pointer for the value
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Using Box<T> Like a Reference
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Use our own smart pointer
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // type `MyBox<{integer}>` cannot be dereferenced
}

// Defining Our Own Smart Pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Treating a Type Like a Reference by Implementing the Deref Trait
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/* 
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("The list is : {:?}", list)
}

*/



