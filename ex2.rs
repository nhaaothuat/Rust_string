fn exercise2() -> String {
  let mut s = String::from("hello");
  s.push(',');
  s.push_str(" world");
  s.push('!');
  s
}

fn main(){
 println!("{}",exercise2());
}
