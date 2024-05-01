fn main() {
    println!("Hello, world!");
    let  x = 5;
    println!("The value of x is: {}", x);
    let y = 7;
    println!("The value of x is: {}", x);
    
    let c = add(x,y);
    println!("The value of c is: {}", c);
}

fn add(a: i32, b: i32)-> i32{
  a+b
}

