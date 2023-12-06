pub fn working_with_references_work(){
    let x = 10;
    let r = &x;             // &x is a shared reference to x
    assert!(*r == 10);  

    // the . operator implicitly dereferences its left operand, if needed
    let aria = Anime { name: "Aria: The Animation", bechdel_pass: true };
    let anime_ref = &aria;

    assert_eq!(anime_ref.name, "Aria: The Animation");

    // Equivalent to the above, but with the dereference written out:
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    // The . operator implicitly borrow a reference to its left operand
    let mut v = vec![1973, 1968];
    v.sort();           // implicitly borrows a mutable reference to v
    (&mut v).sort();    // equivalent, but more verbose
    println!("The sorted vec is : {:?}", v);

    // Assigning references
    let x = 10;
    let y = 20;
    let mut r = &x;

    r = &y; 

    assert!(*r == 10 || *r == 20);

    // Rust permits references to references:
    
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;

    // The . operator follows as many references as it takes to find its target:
    assert_eq!(rrr.y, 729);

    // Comparing References
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rx == ry);              // their referents are equal
    assert!(!std::ptr::eq(rx, ry)); // but occupy different addresses

    assert!(rrx <= rry);
    assert!(rrx == rry);

    // Borrowing References to Arbitrary Expressions
    let r = &factorial(6); // reference to 720 value
    // Arithmetic operators can see through one level of references.
    assert_eq!(r + &1009, 1729);


}

fn factorial(n: usize) -> usize {
    (1..n+1).product()
}


struct Anime { 
    name: &'static str, 
    bechdel_pass: bool 
}
struct Point { 
    x: i32, 
    y: i32 
}