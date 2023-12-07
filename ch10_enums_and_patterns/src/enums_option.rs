pub fn enums_option_work(){
    let some_number = Some(5);
    let some_char:Option<char> = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // Option<T> and T are different types
    // let sum = x + y;
}