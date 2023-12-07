#![allow(dead_code, unused_variables)]
mod enums;
mod patterns;
mod enums_book;
mod enums_option;
mod matchs_book;
mod iflet;

use crate::enums::enums_work;
use crate::patterns::patterns_work;
use crate::enums_book::enums_book_work;
use crate::enums_option::enums_option_work;
use crate::matchs_book::matchs_book_work;
use crate::iflet::iflet_work;

fn main() {
    println!("Hello, world!");
    enums_work();
    patterns_work();
    enums_book_work();
    enums_option_work();
    matchs_book_work();
    iflet_work();
}

