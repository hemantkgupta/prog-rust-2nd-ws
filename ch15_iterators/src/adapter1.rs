pub fn adapter1_work(){
    let text = "  ponies  \n   giraffes\niguanas  \nsquid".to_string();

    let v: Vec<&str> = text.lines()
        /*
        core::iter::traits::iterator::Iterator
        pub fn map<B, F>(self, f: F) -> Map<Self, F>
        where
            Self: Sized,
            F: FnMut(Self::Item) -> B,
         */
        .map(str::trim)
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);

    let text = "  ponies  \n   giraffes\niguanas  \nsquid".to_string();

    let v: Vec<&str> = text.lines()
        .map(str::trim)
        /*
        core::iter::traits::iterator::Iterator
        pub fn filter<P>(self, predicate: P) -> Filter<Self, P>
        where
            Self: Sized,
            P: FnMut(&Self::Item) -> bool,
         */
        .filter(|s| *s != "iguanas")
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);

    use std::str::FromStr;

    let text = "1\nfrond .25  289\n3.1415 estuary\n";
    for number in text
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok())
    {
        println!("{:4.2}", number.sqrt());
    }

    use std::collections::BTreeMap;

    // A table mapping cities to their parks: each value is a vector.
    let mut parks = BTreeMap::new();
    parks.insert("Portland",  vec!["Mt. Tabor Park", "Forest Park"]);
    parks.insert("Kyoto",     vec!["Tadasu-no-Mori Forest", "Maruyama Koen"]);
    parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);

    // Build a vector of all parks. `values` gives us an iterator producing
    // vectors, and then `flatten` produces each vector's elements in turn.
    let all_parks: Vec<_> = parks.values().flatten().cloned().collect();

    assert_eq!(all_parks,
            vec!["Tadasu-no-Mori Forest", "Maruyama Koen", "Percy Warner Park",
                    "Dragon Park", "Mt. Tabor Park", "Forest Park"]);

    assert_eq!(vec![None, Some("day"), None, Some("one")]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>(),
    vec!["day", "one"]);

    let message = "To: jimb\r\n\
               From: superego <editor@oreilly.com>\r\n\
               \r\n\
               Did you get any writing done today?\r\n\
               When will you stop wasting time plotting fractals?\r\n";

    for header in message.lines().take_while(|l| !l.is_empty()) {
        println!("{}" , header);
    }

    for body in message.lines()
    .skip_while(|l| !l.is_empty())
    .skip(1) {
    println!("{}" , body);

    let id = "Iterator";

    assert!( id.chars().any(char::is_uppercase));
    assert!(!id.chars().all(char::is_uppercase));

    let a = [5, 6, 7, 8, 9, 10];

    assert_eq!(a.iter().fold(0, |n, _| n+1), 6);        // count
    assert_eq!(a.iter().fold(0, |n, i| n+i), 45);       // sum
    assert_eq!(a.iter().fold(1, |n, i| n*i), 151200);   // product

    // max
    assert_eq!(a.iter().cloned().fold(i32::min_value(), std::cmp::max),
           10);
    
}


}

/*
fn to_uppercase(&self) -> String {
    self.chars()
        .map(char::to_uppercase)
        .flatten() // there's a better way
        .collect()
}

fn to_uppercase(&self) -> String {
    self.chars()
        .flat_map(char::to_uppercase)
        .collect()
}

 */