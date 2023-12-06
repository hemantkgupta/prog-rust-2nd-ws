pub fn move_work(){

    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s.clone();
    println!("The t is: {:?}", t);
    let u = s.clone();
    println!("The u is: {:?}", u);

    let mut s = "Govinda".to_string();
    let _t = s;
    s = "Siddhartha".to_string(); // nothing is dropped here

    // Build a vector of the strings "101", "102", ... "105"
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }

    // Each one of these methods moves an element out of the vector,
    // but does so in a way that leaves the vector in a state that is fully populated

    // 1. Pop a value off the end of the vector:
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    // 2. Move a value out of a given index in the vector,
    // and move the last element into its spot:
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // 3. Swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    // Let's see what's left of our vector.
    assert_eq!(v, vec!["101", "104", "substitute"]);

    let v = vec!["liberté".to_string(),
             "égalité".to_string(),
             "fraternité".to_string()];

    for mut s in v {
        s.push('!');
        println!("{}", s);
    }

    // v is moved
    // println!("The vector is : {:?}", v);

    let v = vec!["liberté".to_string(),
             "égalité".to_string(),
             "fraternité".to_string()];

    for s in &v {
        //s.push('!');
        println!("{}", s);
    }

    println!("The vector is : {:?}", v);

    struct Person { name: Option<String>, birth: i32 }

    let mut composers = Vec::new();
    composers.push(Person { name: Some("Palestrina".to_string()),
                            birth: 1525 });

    let first_name = std::mem::replace(&mut composers[0].name, None);
    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);

}   