use std::io::Write;
use std::fs::File;



fn say_hello1(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n").unwrap();
    out.flush()
}
fn say_hello2<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n").unwrap();
    out.flush()
}

fn say_hello3<W>(out: &mut W) -> std::io::Result<()>
where W: Write
{
    out.write_all(b"hello world\n").unwrap();
    out.flush()
}



pub fn using_traits_work(){

    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"hello"); 
    let _ = buf.flush();
    assert_eq!(buf, b"hello");


    let mut local_file = File::create("hello.txt").expect("Error encountered while creating file!");
    let _ = say_hello1(&mut local_file);  

    let mut bytes = vec![];
    let _ = say_hello1(&mut bytes); 
    assert_eq!(bytes, b"hello world\n");

    let mut bytes = vec![];
    let _ = say_hello3(&mut bytes); 
    assert_eq!(bytes, b"hello world\n");

}