use std::rc::Rc;
pub fn rc_arc_work(){

    // Rust can infer all these types; written out for clarity
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);
    example();
}

#[derive(Debug)]
struct Pet {
    name: String,
}

impl Pet {
    fn new(name: String) -> Self {
        Self { name }
    }
}

struct Person {
    pets: Vec<Rc<Pet>>,
}

fn example(){
    // Create two pets with shared ownership
    let cat = Rc::new(Pet::new("Tigger".to_string()));
    let dog = Rc::new(Pet::new("Chase".to_string()));

    // Create one person who owns both pets
    let brother = Person {
        pets: vec![cat.clone(), dog.clone()],
    };

    // Create another person who _also_ owns both pets
    let sister = Person {
        pets: vec![cat, dog],
    };

    // Even if one person gives up ownership,
    // the other person still has shared ownership,
    // so the pets are kept around (yay!)
    drop(sister);
    println!("Pets: {:?}", brother.pets)
}
