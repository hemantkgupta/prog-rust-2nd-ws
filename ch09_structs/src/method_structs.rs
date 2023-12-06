pub fn method_struts_work(){

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Same name method as field
    fn width(&self) -> bool {
        self.width > 0
    }

    // Method with more parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

}
