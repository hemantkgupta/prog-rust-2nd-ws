use std::path::PathBuf;
use std::{fs, thread};
use std::sync::mpsc;

pub fn channels_mpsc_work(){
    let path1 = PathBuf::from("./file1.txt");
    let path2 = PathBuf::from("./file2.txt");
    let path3 = PathBuf::from("./file3.txt");
    let documents: Vec<PathBuf> = vec![path1, path2, path3];
    let (texts,   h1) =  start_file_reader_thread(documents);
    let _r1 = h1.join().unwrap();
    println!("Done with reading files.");
    for text in texts {
        println!("Printing");
        println!("The text is: {}",text);
    }
}
fn start_file_reader_thread(documents: Vec<PathBuf>) 
    -> (mpsc::Receiver<String>, thread::JoinHandle<std::io::Result<()>>)
    {
        let (sender, receiver) = mpsc::channel();
        
        let handle = thread::spawn(move || {
            for filename in documents {
                println!("Started reading file text.");
                let text = fs::read_to_string(filename)?;
                println!("Done reading file text: {}", text);
                if sender.send(text).is_err() {
                    println!("Err in sending text.");
                    break;
                }
            }
            Ok(())
        });
    
    (receiver, handle)
}