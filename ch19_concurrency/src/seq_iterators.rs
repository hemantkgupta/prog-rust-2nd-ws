pub fn seq_iterators_work(){
    let names = vec!["Alice".to_string(), "Bob".to_string()]; 
    let is_present = names.iter().any(|name| name == "Alice"); 
    println!("The data is found: {}", is_present);
}

