use std::fmt;

pub fn trait_newtype_work(){

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    
    /* let w2 = WrapperInt(vec![1, 2, 3]);
    println!("w = {}", w2); */

}


struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

/* 
struct WrapperInt(Vec<i32>);

impl fmt::Display for WrapperInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}]", self.0)
    }
}
*/

