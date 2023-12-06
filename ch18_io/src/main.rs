mod networking1;
use crate::networking1::networking1_work;

mod networking2;
use crate::networking2::networking2_work;
mod networking0;
use crate::networking0::networking0_work;

fn main() {
    println!("Hello, world!");
    networking1_work();
    networking2_work();
    networking0_work();
}
