use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::fs;

pub fn results_work(){
    //example1();
    // example2();
    // example3();
    // example4();
    // example5();
    // example6();
    //let result = example7();
    // println!("The result is: {:?}", result);
    // let result = example8();
    // println!("The result is: {:?}", result);
    // let result = example9();
    // println!("The result is: {:?}", result);
    // let result = example10();
    // println!("The result is: {:?}", result);
    let result = example11();
    println!("The result is: {:?}", result);
}
fn example1(){
    let greeting_file_result = File::open("hello.txt");
    match greeting_file_result {
        Ok(_) => println!("Successfully opened the file."),
        Err(_) => println!("Failed to open the file."),
    };
}
fn example2(){
    let greeting_file_result = File::open("hello1.txt");
    match greeting_file_result {
        Ok(_) => println!("Successfully opened the file."),
        Err(err) =>  match err.kind() {
            ErrorKind::NotFound => println!("File not found."),
            other_error => println!("Not a file not found error."),
        },  
    };
}

fn example3(){
    let greeting_file_result = File::open("hello1.txt");
    match greeting_file_result {
        Ok(_) => println!("Successfully opened the file."),
        Err(err) =>  match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(_) => println!("Successfully created the file."),
                Err(_) => panic!("Problem creating the file."),
            },
            other_error => println!("Not a file not found error."),
        },  
    };
}
// Shortcuts for Panic on Error: unwrap and expect
// Also use closure inplace of match
fn example4(){
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn example5(){
    // Will panic if couldn't open the file
    let greeting_file = File::open("hello1.txt").unwrap();
}

fn example6(){
    // Will panic and print message if couldn't open the file
    let greeting_file = File::open("hello1.txt")
        .expect("hello.txt should be included in this project");
}

fn example7() -> Result<String, std::io::Error> {
    let greeting_file_result = File::open("hello.txt");
    match greeting_file_result {
        Ok(_) => Ok("Successfully opened the file.".to_string()),
        Err(err) =>  return Err(err),
    }
}

fn example8() -> Result<String, std::io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn example9() -> Result<String, std::io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn example10() -> Result<String, std::io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn example11() -> Result<String, std::io::Error> {
    fs::read_to_string("hello.txt")
}
