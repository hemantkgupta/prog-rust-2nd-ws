#![allow(unused_imports, dead_code, unused_variables)]
mod panics;
use crate::panics::panics_work;
mod results;
use crate::results::results_work;
fn main() {
    println!("Hello, world!");
    //panics_work();
    results_work();
}
