fn main() {
    let width: u32 = 30;
    let length: u32 = 50;
    
    let area = calculate_area1(width, length);
    println!("Area of rectangle is {} square pixels", area);
}

fn calculate_area1(width: u32, length: u32) -> u32{
    return width*length;
}
