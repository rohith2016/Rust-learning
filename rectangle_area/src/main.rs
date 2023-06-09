#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            width: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        length: 50,
        width: 30,
    };

    let rect2: Rectangle = Rectangle {
        length: 40,
        width: 15,
    };
    let square = Rectangle::square(20);
    println!("rect: {:#?}", rect);
    println!("Area of rectangle is {} square pixels", rect.area());
    println!("Area of rectangle is {} square pixels", rect2.area());
    println!("Rect can hold Rect2: {}", rect.can_hold(&rect2));
    print!("Area of square is {} square pixels",  square.area());
}
