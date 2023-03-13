use rand::Rng;
use colored::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Guessing game!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret Number: {}", secret_number);
    loop {
        println!("Input any number between 1 to 100");
        let mut _guess: String = String::new();
        io::stdin()
            .read_line(&mut _guess)
            .expect("Failed to read  line");
        let guess: u32 = match _guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        print!("You guessed: {}", _guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Equal => {
                println!("{}","Perfect".green());
                break;
            },
            Ordering::Greater => println!("{}", "Too big".red()),
        }
    }
}
