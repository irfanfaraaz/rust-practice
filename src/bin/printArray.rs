fn main(){
  let arr = [1,2,3,4,5];
  println!("{:?}",arr);

  for i in arr.iter() {
    println!("{}", i);
  }
}