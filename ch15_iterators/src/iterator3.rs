pub fn iterator3_work(){
    let mut outer = "Earth".to_string();

    // Drain on 1..4 (index 1, 2 and 3)from outer
    let inner = String::from_iter(outer.drain(1..4));

    assert_eq!(outer, "Eh");
    assert_eq!(inner, "art");
}