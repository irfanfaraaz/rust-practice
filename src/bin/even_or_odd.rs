use std::io;

fn main() {
    println!("Enter a number");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Please type a number!");
    if n % 2 == 0 {
        println!("The number {} is even", n);
    } else {
        println!("The number {} is odd", n);
    }
}
