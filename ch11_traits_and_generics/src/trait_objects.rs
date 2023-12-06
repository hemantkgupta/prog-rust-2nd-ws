use std::io::Write;
use std::hash::Hash;
use std::fmt::Debug;


pub fn trait_objects_work(){
    let mut buf: Vec<u8> = vec![];
    let writer: &mut dyn Write = &mut buf;
}

fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) { 
    //
}

trait Vegetable {
}


struct Salad {
    veggies: Vec<Box<dyn Vegetable>>
}