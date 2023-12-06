#![warn(unused_imports)]
mod vec;
use crate::vec::vic_work;

mod vecdeq;
use crate::vecdeq::vecdeq_work;

mod binaryheap;
use crate::binaryheap::binaryheap_work;

//mod hashmap;
//use crate::hashmap::hashmap_work;

fn main() {
    println!("Hello, world!");
    vic_work();
    vecdeq_work();
    binaryheap_work();
    //hashmap_work();
}
