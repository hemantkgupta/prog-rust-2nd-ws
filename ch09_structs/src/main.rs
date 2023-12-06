#![allow(dead_code, unused_variables)]
mod named_structs;
mod tuple_structs;
mod unit_structs;
mod associated_const;
mod generic_structs;
mod lifetime_structs;
mod generic_const_structs;
mod method_impl;
mod structs_book;
mod derive_traits_structs;
mod method_structs;

use crate::named_structs::named_work;
use crate::tuple_structs::tuple_structs_work;
use crate::unit_structs::unit_structs_work;
use crate::associated_const::associated_const_work;
use crate::generic_structs::generic_structs_work;
use crate::lifetime_structs::lifetime_structs_work;
use crate::generic_const_structs::generic_const_structs_work;
use crate::structs_book::structs_book_work;
use crate::derive_traits_structs::derive_traits_structs_work;
use crate::method_structs::method_struts_work;

fn main() {
    println!("Hello, world!");
    named_work();
    tuple_structs_work();
    unit_structs_work();
    associated_const_work();
    generic_structs_work();
    lifetime_structs_work();
    generic_const_structs_work();
    structs_book_work();
    derive_traits_structs_work();
    method_struts_work();

}   
