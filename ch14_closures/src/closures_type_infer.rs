pub fn closure_type_infer_work(){
    let example_closure = |x| x;

    let _s = example_closure(String::from("hello"));
    
    // expected `String`, found integer
    // let n = example_closure(5);
}