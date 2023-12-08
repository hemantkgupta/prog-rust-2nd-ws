use std::fs::File;
pub fn result1_work(){
    all_single_steps();
    use_unwrap_on_result();
}

fn all_single_steps(){
    let f: Result<File, std::io::Error> = File::open("file0.txt"); 

    if f.is_err(){
        let err: Option<std::io::Error> = f.err();
        let er: std::io::Error = err.unwrap();

        let e:String = er.to_string();
        println!("{:?}",e);
    } else {
        println!("Successfully opened the file.")
    }
    

}

fn use_unwrap_on_result(){
    let f: Result<File, std::io::Error> = File::open("file1.txt"); 
    if f.is_ok(){
        println!("Successfully opened the file.");
        let _e1: File = f.unwrap();
    }
}