#![allow(unused_imports, unused_variables, unused_mut)]
mod vec;
use crate::vec::vic_work;

mod vecdeq;
use crate::vecdeq::vecdeq_work;

mod binaryheap;
use crate::binaryheap::binaryheap_work;

mod hashmap_book;
use crate::hashmap_book::hashmap_book_work;

mod string_book;
use crate::string_book::string_book_work;

fn main() {
    println!("Hello, world!");
    vic_work();
    vecdeq_work();
    binaryheap_work();
    hashmap_book_work();
    string_book_work();
}
