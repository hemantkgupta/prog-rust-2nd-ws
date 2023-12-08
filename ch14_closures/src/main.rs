mod closures_book;
mod closures_type_infer;
mod closures_move;

use crate::closures_book::closures_book_work;
use crate::closures_type_infer::closure_type_infer_work;
use crate::closures_move::closures_move_work;
fn main() {
    println!("Hello, world!");
    closures_book_work();
    closure_type_infer_work();
    closures_move_work();
}
