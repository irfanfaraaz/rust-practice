use std::io;
fn main(){
  println!("Enter a number: ");
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");
  let n:i32 = input.trim().parse().expect("Please type a number!");
  let f = factorial(n);
  println!("The factorial of {} is: \n{}", n, f);
}

fn factorial(n:i32) ->i32 {
  if n==0 {
    1
  }
 else{
   n*factorial(n-1)
 }
}