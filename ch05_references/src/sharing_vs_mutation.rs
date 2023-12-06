pub fn sharing_vs_mutation_work(){
    /* 
    let v = vec![4, 8, 19, 27, 34, 10];
    let r = &v;
    let aside = v;  // move vector to aside
    r[0];  // // bad: uses `v`, which is now uninitialized
    */

    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head);   // extend wave with another vector
    extend(&mut wave, &tail);   // extend wave with an array

    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);

    // we may borrow a mutable reference to the vector, 
    // and we may borrow a shared reference to its elements, 
    // but those two references’ lifetimes must not overlap. 
    // extend(&mut wave, &wave);
    // assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0]);

    let x = 10;
    let r1 = &x;
    let r2 = &x;     // ok: multiple shared borrows permitted
    // x += 10;         // error: cannot assign to `x` because it is borrowed
    // let m = &mut x;  // error: cannot borrow `x` as mutable because it is
                    // also borrowed as immutable
    // println!("{}, {}, {}", r1, r2, m); // the references are used here,
                                   // so their lifetimes must last
                                   // at least this long

    let mut y = 20;
    let m1 = &mut y;
    // let m2 = &mut y;  // error: cannot borrow as mutable more than once
    // let z = y;        // error: cannot use `y` because it was mutably borrowed
    // println!("{}, {}, {}", m1, m2, z); // references are used here

    // It is OK to reborrow a shared reference from a shared reference
    let  w = (107, 109);
    let r = &w;
    let r0 = &r.0;         // ok: reborrowing shared as shared
    // let m1 = &mut r.1;     // error: can't reborrow shared as mutable
    println!("{}", r0);    // r0 gets used here

    // You can reborrow from a mutable reference:
    let mut v = (136, 139);
    let m = &mut v;
    let m0 = &mut m.0;      // ok: reborrowing mutable from mutable
    *m0 = 137;
    let r1 = &m.1;          // ok: reborrowing shared from mutable,
                            // and doesn't overlap with m0
    // v.1;                    // error: access through other paths still forbidden
    // println!("{}", r1);     // r1 gets used here


}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}