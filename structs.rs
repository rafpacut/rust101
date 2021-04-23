#[derive(Debug)]
struct Rectangle {
        width: u32,
        height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle::square(10);
    let rect2 = Rectangle::square(20);
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
