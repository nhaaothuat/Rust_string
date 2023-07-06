fn count_char_occurrences(string: &str, ch: char) -> usize {
  let mut count = 0;
  for c in string.chars(){
    if c == ch {
     count +=1;
   }
 }
  count
}

fn main(){
  println!("{}", count_char_occurrences("Hello", 'l'));
  println!("{}", count_char_occurrences("Rust is fun", 'u'));
  println!("{}", count_char_occurrences("Mississippi", 's'));
}
