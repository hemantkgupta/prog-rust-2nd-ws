#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(dead_code)]
mod using_traits;
mod trait_objects;
mod trait_newtype;
mod trait_supertrait;
mod generics_book;
mod generics_structs;
mod generics_method;
mod traits_book;

use crate::using_traits::using_traits_work;
use crate::trait_objects::trait_objects_work;
use crate::trait_newtype::trait_newtype_work;
use crate::trait_supertrait::trait_supertrait_work;
use crate::generics_book::generics_book_work;
use crate::generics_structs::generics_structs_work;
use crate::generics_method::generics_method_work;
use crate::traits_book::traits_book_work;

fn main() {
    println!("Hello, world!");
    using_traits_work();
    trait_objects_work();
    trait_newtype_work();
    trait_supertrait_work();
    generics_book_work();
    generics_structs_work();
    generics_method_work();
    traits_book_work();
    
}
