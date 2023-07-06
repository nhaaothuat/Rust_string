fn exercise3() -> String {
  let s1 = String::from("hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2;
  s3
}

fn main(){
  println!("{}",exercise3());
}
