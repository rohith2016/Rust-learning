#[derive(Debug)]
struct Rectangle{
    length: u32,
    width: u32
}

fn main() {
    let dimensions = Rectangle{
        length: 50,
        width: 30,
    };
    println!("rect: {:#?}", dimensions);
    println!("Area of rectangle is {} square pixels", calculate_area(&dimensions));
}

//pass reference to Rectangle as we want to access its fields but not take ownership
fn calculate_area(dimensions: &Rectangle) -> u32{
    dimensions.length*dimensions.width
}
