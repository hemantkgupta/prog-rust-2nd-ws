pub fn iterator1_work(){

    let v = [1, 2, 3];
    let mut iter = v.into_iter();

    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(2), iter.next());
    assert_eq!(Some(3), iter.next());
    assert_eq!(None, iter.next());

    let v = vec![4, 20, 12, 8, 6];

    // This iterator’s item type is &i32 not i32
    // Each call to next produces a reference
    let mut iterator = v.iter();

    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&12));
    assert_eq!(iterator.next(), Some(&8));
    assert_eq!(iterator.next(), Some(&6));
    assert_eq!(iterator.next(), None);

    println!("There's:");
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];

    for element in &v {
        println!("{}", element);
    }

    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }

    use std::ffi::OsStr;
    use std::path::Path;

    let path = Path::new("C:/Users/JimB/Downloads/Fedora.iso");
    let mut iterator = path.iter();
    assert_eq!(iterator.next(), Some(OsStr::new("C:")));
    assert_eq!(iterator.next(), Some(OsStr::new("Users")));
    assert_eq!(iterator.next(), Some(OsStr::new("JimB")));

    use std::collections::BTreeSet;
    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the Sky With Diamonds".to_string());
    favorites.insert("Liebesträume No. 3".to_string());

    let mut it = favorites.into_iter();
    assert_eq!(it.next(), Some("Liebesträume No. 3".to_string()));
    assert_eq!(it.next(), Some("Lucy in the Sky With Diamonds".to_string()));
    assert_eq!(it.next(), None);


}