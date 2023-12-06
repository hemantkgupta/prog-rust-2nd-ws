#![allow(unused_imports)]
mod arithmatic;
mod index_indexmut;
mod eq_ord;

use crate::arithmatic::arithmatic_work;
use crate::index_indexmut::index_indexmut_work;
use crate::eq_ord::eq_ord_work;
fn main() {
    println!("Hello, world!");
    arithmatic_work();
    index_indexmut_work();
    eq_ord_work();
}
