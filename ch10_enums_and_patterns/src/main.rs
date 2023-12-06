#![allow(dead_code)]
#![allow(unused_variables)]
mod enums;
mod patterns;

use crate::enums::enums_work;
use crate::patterns::patterns_work;


fn main() {
    println!("Hello, world!");
    enums_work();
    patterns_work();
}

