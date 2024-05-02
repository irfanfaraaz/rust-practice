fn main(){
  println!("Odd numbers from 1 to 10: ");
  count();
  println!("Even numbers from 1 to 20: ");
  even();
}

fn count(){
  for i in 1..11 {
    println!("{}",i);
  }
}

fn even(){
  for i in 1..21 {
    if i%2 ==0 {
      println!("{}",i);
    }
  }
}