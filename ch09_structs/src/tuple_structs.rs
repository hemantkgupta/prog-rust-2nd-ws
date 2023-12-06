pub fn tuple_structs_work(){
    let image_bounds = Bounds(1024, 768);
    assert_eq!(image_bounds.0 * image_bounds.1, 786432);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct Bounds(usize, usize);

// Elements of a tuple-like struct may be public or not
pub struct Bounds2(pub usize, pub usize);

// Defining the type also implicitly defines a function
// fn Bounds(elem0: usize, elem1: usize) -> Bounds { ... }

// Use this type for your ASCII strings
// It is much better than Vec<u8>
struct Ascii(Vec<u8>);

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);