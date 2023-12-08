mod iterator1;
use crate::iterator1::iterator1_work;

mod iterator2;
use crate::iterator2::iterator2_work;

mod iterator3;
use crate::iterator3::iterator3_work;

mod adapter1;
use crate::adapter1::adapter1_work;

mod iterator4;
use crate::iterator4::iterator4_work;

mod iterators_book;
use crate::iterators_book::iterators_book_work;


fn main() {
    println!("Hello, world!");
    iterator1_work();
    iterator2_work();
    iterator3_work();
    adapter1_work();
    iterator4_work();
    iterators_book_work();
}
