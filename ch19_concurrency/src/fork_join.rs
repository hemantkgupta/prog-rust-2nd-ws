use std::{fs, thread, panic};

pub fn fork_join_work(){
    match copy_in_thread() {
        Ok(_) => println!("copy succeeded"),
        Err(e) => panic::resume_unwind(e),
    }

    match copy_in_thread2() {
        Ok(_) => println!("Ok. copied"),
        Err(e) => println!("Error in copying file : {:?}", e),
    }
}



fn copy_in_thread() -> thread::Result<()> {
    let handle  = thread::spawn(|| {
        // unwrap asserting that it is an Ok result and not an Err result. 
        // If a child thread did panic, then this assertion would fail,
        // so the parent thread would panic too
        fs::copy("foo.txt", "bar.txt").unwrap();
    });
    handle.join()
}

fn copy_in_thread2() -> thread::Result<()> {
    thread::spawn(|| {
        fs::copy("foo.txt", "bar.txt").expect("Error occurred");
    })
    .join()
}