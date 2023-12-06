use async_std::task;
pub fn async1_work(){
    task::block_on(say_hello())
}

async fn say_hello() {
    println!("Hello, world!");
}
