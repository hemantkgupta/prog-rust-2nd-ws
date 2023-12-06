use std::ops::Add;
pub fn arithmatic_work(){
    let a = 3;
    let b = 5;
    let c;
    c = a.add(b);
    assert_eq!(c, 8);

    assert_eq!(4.125f32.add(5.75), 9.875);
    assert_eq!(10.add(20), 10 + 20);
    
    // Example
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}


#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


struct Millimeters(u32);
struct Meters(u32);

// Implementing trait add for Millimeters
// The Rhs is Meters
// Output is Millimeters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
