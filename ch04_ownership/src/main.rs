#![allow(unused_assignments, dead_code)]
mod moves;
use crate::moves::move_work;
mod copy_types;
use crate::copy_types::copy_types_work;

mod rc_arc;
use crate::rc_arc::rc_arc_work;

fn main() {
    println!("Hello, world!");
    move_work();
    copy_types_work();
    rc_arc_work();
}
