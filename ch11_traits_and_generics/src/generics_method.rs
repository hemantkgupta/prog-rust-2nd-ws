pub fn generics_method_work(){
    let p = Point { x: 5, y: 10 };
    let p2: Point<f32> = Point { x: 5.0, y: 10.0 };

    println!("p.x = {}", p.x());
    println!("The distance from origin is : {}", p2.distance_from_origin());

    let p3 = Point2 { x: 5, y: 10.4 };
    let p4 = Point2 { x: "Hello", y: 'c' };

    let p5 = p3.mixup(p4);

    println!("p3.x = {}, p3.y = {}", p5.x, p5.y);
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // Method can have same new as field
    fn x(&self) -> &T {
        &self.x
    }    
}

// Restriction on generic type for method
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

// Generic type parameters in a struct definition aren’t always the same
// as those you use in that same struct’s method signatures.
impl<X1, Y1> Point2<X1, Y1> {

    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}
