mod fork_join;
use crate::fork_join::fork_join_work;
mod share_data;
use crate::share_data::share_data_work;
mod seq_iterators;
use crate::seq_iterators::seq_iterators_work;
mod par_rayon;
use crate::par_rayon::par_rayon_work;
mod channels;
use crate::channels::channels_work;
fn main() {
    println!("Hello, world!");
    fork_join_work();
    share_data_work();
    seq_iterators_work();
    par_rayon_work();
    channels_work();
}

