use std::collections::HashSet;

pub fn vic_work(){
    // Create an empty vector
    let numbers: Vec<i32> = vec![];

    // Create a vector with given contents
    let _words = vec!["step", "on", "no", "pets"];
    let buffer = vec![0u8; 1024];  // 1024 zeroed-out bytes

    // Get a copy of an element
    let _fifth_number = numbers[0];       // requires Copy

    // Get a reference to a slice
    let _my_ref = &buffer[4..12];

    // Get a copy of a slice
    let _my_copy = buffer[4..12].to_vec();  // requires Clone

    let slice = [0, 1, 2, 3];
    assert_eq!(slice.get(2), Some(&2));
    assert_eq!(slice.get(4), None);

    let mut slice = [0, 1, 2, 3];
    {
        let last = slice.last_mut().unwrap();   // type of last: &mut i32
        assert_eq!(*last, 3);
        *last = 100;
    }
    assert_eq!(slice, [0, 1, 2, 100]);

    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(v.to_vec(),
           vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(v[0..6].to_vec(),
           vec![1, 2, 3, 4, 5, 6]);
    
    let mut byte_vec = b"Misssssssissippi".to_vec();
    byte_vec.dedup();
    assert_eq!(&byte_vec, b"Misisipi");

    let mut byte_vec = b"Misssssssissippi".to_vec();

    let mut seen = HashSet::new();
    byte_vec.retain(|r| seen.insert(*r));

    assert_eq!(&byte_vec, b"Misp");

    assert_eq!([[1, 2], [3, 4], [5, 6]].concat(),
           vec![1, 2, 3, 4, 5, 6]);

    assert_eq!([[1, 2], [3, 4], [5, 6]].join(&0),
           vec![1, 2, 0, 3, 4, 0, 5, 6]);
    
    // Splitting
    let v = vec![0, 1, 2, 3];
    let _a = &v[1];
    let _b = &v[2];

    let mid = v.len() / 2;
    let _front_half = &v[..mid];
    let _back_half = &v[mid..];

    assert_eq!([1, 2, 3, 4].starts_with(&[1, 2]), true);
    assert_eq!([1, 2, 3, 4].starts_with(&[2, 3]), false);


}