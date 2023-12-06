pub fn copy_types_work(){
    let l = Label { number: 3 };
    println!("{:?}", l);
    println!("My label number is: {}", l.number);
}

#[derive(Debug)]
struct Label { number: u32 }

// error[E0204]: the trait `Copy` cannot be implemented for this type
// #[derive(Copy, Clone)] 
// struct StringLabel { name: String }