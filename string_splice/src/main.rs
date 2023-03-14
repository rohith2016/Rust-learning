fn main() {
    let s = String::from("Hello World");
    let word = first_word(&s);

    println!("{}", word);
}

fn first_word(some_string: &str) -> &str{
    let bytes = some_string.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &some_string[..i];
        }
    }

    return  &some_string[..];
}
