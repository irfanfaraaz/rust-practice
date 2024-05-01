use std::io;

fn main(){
    println!("Enter the first number: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let a: i32 = input1.trim().parse().expect("Please type a number!");

    println!("Enter the second number: ");
    input1 = String::new();
  io::stdin().read_line(&mut input1).expect("Failed to read line");
  let b:i32 = input1.trim().parse().expect("Please type a number!");

    let c = a+b;
    println!("The value of c is: {}", c);
}