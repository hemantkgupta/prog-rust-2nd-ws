#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(dead_code)]
mod using_traits;
mod trait_objects;
mod trait_newtype;
mod trait_supertrait;

use crate::using_traits::using_traits_work;
use crate::trait_objects::trait_objects_work;
use crate::trait_newtype::trait_newtype_work;
use crate::trait_supertrait::trait_supertrait_work;

fn main() {
    println!("Hello, world!");
    using_traits_work();
    trait_objects_work();
    trait_newtype_work();
    trait_supertrait_work();
}
