#![allow(dead_code)]
mod async1;
use crate::async1::async1_work;
mod async2;
use crate::async2::async2_work;
mod async3;
use crate::async3::async3_work;
mod async4;
use crate::async4::async4_work;
mod async5;
use crate::async5::async5_work;
mod async6;
use crate::async6::async6_work;


fn main() {
    println!("Hello,  Async world!");
    async1_work();
    let _ = async2_work();
    async3_work();
    let _ = async4_work();
    let _ = async5_work();
    //async5_work();
    async6_work();
}
