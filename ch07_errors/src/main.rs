#![allow(unused_imports, dead_code, unused_variables)]
mod panics;
use crate::panics::panics_work;
mod results;
use crate::results::results_work;

mod result1;
use crate::result1::result1_work;
fn main() {
    println!("Hello, world!");
    //panics_work();
    //results_work();
    result1_work();
}
