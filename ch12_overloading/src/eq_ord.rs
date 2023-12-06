use std::cmp::Ordering;
use std::ops::Not;

pub fn eq_ord_work(){
    println!("Points & Lines");
    let point_a: Point = Point::new(0.0, 0.0);
    let point_b: Point = Point::new(0.0, 10.0);
    let line_a: Line = Line::new(point_a, point_b);

    println!("line a has a length of {:?}", line_a.length());

    let point_c: Point = Point::new(10.0, 10.0);
    let point_d: Point = Point::new(10.0, 20.0);
    let line_b: Line = Line::new(point_c, point_d);

    println!("line b has a length of  {:?}", line_b.length());
    println!("are line a and b lengths equal? {:?}", line_a == line_b);
    println!(
        "is the mirror line's a length equal to line b {:?}",
        !line_a == line_b
    );

    let line_c: Line = Line::new(point_a, point_d);
    let line_d: Line = Line::new(point_b, point_c);

    println!("line c has a length of {:?}", line_c.length());
    println!("line d has a length of {:?}", line_d.length());
    println!("is line c smaller than line b? {:?}", line_c < line_b);

}
#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

pub trait PointProperties {
    fn new(x: f32, y: f32) -> Self;
}

impl PointProperties for Point {
    fn new(x: f32, y: f32) -> Self {
        return Self { x: x, y: y };
    }
}

pub trait LineProperties {
    fn length(&self) -> f32;
    fn new(a: Point, b: Point) -> Self;
}

impl LineProperties for Line {
    fn length(&self) -> f32 {
        return ((&self.end.x - &self.start.x).powf(2.0) + (&self.end.y - &self.start.y).powf(2.0))
            .sqrt();
    }
    fn new(a: Point, b: Point) -> Self {
        return Self { start: a, end: b };
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        return &self.length() == &other.length();
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.length().partial_cmp(&other.length())
    }
}

impl Not for Line {
    type Output = Line;
    fn not(mut self) -> Line {
        self.start.x = -self.start.x;
        self.start.y = -self.start.y;
        self.end.x = -self.end.x;
        self.end.y = -self.end.y;

        return self;
    }
}


