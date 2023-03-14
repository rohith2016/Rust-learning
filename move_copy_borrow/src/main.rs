fn main() {
    let s = String::from("Hello");
    take_ownership(s);
    //  println!("{}", s); -> will give error

    let s2 = String::from("World");
    let len = calculate_length_via_borrowing(&s2);
    println!("the length of {} is {}", s2, len);

    let mut s3 = String::from("Hello");
    change(&mut s3);
    println!("{}", s3);

    let i = 5;
    makes_copy(i);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length_via_borrowing(some_string2: &String) -> usize {
    let len = some_string2.len();
    len
}

fn change(some_string: &mut String) {
    some_string.push_str(", World");
}
