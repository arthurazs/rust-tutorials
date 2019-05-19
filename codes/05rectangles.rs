#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }

impl Rectangle {
    fn area(&self) -> u32 { self.width * self.height }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(dimension: u32) -> Rectangle {
        Rectangle { width: dimension, height: dimension }
    }
}

fn main() {

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let squa1 = Rectangle::square(25);

    println!(
        "The area of the {:?} is {} square pixels.",
        squa1, squa1.area());

    println!(
        "Can {:?} hold {:?}? {}",
        rect1, rect2, rect1.can_hold(&rect2));

    println!(
        "Can {:?} hold {:?}? {}",
        rect1, rect3, rect1.can_hold(&rect3));
}
