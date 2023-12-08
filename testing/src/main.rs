#![allow(dead_code, unused_variables)]
mod test1;
mod panic_test;
mod results_test;
mod private_tests;
mod function_output_tests;
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}