fn main(){
  let  a = 7;
  let string_num = a.to_string();
  println!("the parsed int is {}", string_num);
  let string_val ="777";
  let parsed_int :i32 = string_val.trim().parse().expect("not a number");
  println!("the parsed string is {}", parsed_int);
}