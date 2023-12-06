#![allow(dead_code, unused_assignments, unused_variables)]
mod smart_pointers;
use crate::smart_pointers::smart_pointers_work;

mod reference_to_value;
use crate::reference_to_value::reference_to_value_work;

mod working_with_references;
use crate::working_with_references::working_with_references_work;

mod reference_safety;
use crate::reference_safety::reference_safety_work;

mod sharing_vs_mutation;
use crate::sharing_vs_mutation::sharing_vs_mutation_work;

fn main() {
    println!("Hello, world!");
    smart_pointers_work();
    reference_to_value_work();
    working_with_references_work();
    reference_safety_work();
    sharing_vs_mutation_work();
}
