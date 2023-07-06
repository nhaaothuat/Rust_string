fn is_palindrome(word: &str) -> bool {
// let nor = word.chars().filter(|c| c.is_palindrome()).collect::<String>();
// nor.chars().eq(normalized.chars().rev()) 

let word = word.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    let reversed = word.chars().rev().collect::<String>();

    word.eq_ignore_ascii_case(&reversed)
}

fn main(){
  println!("{}",is_palindrome("level"));
  println!("{}",is_palindrome("deed"));
  println!("{}",is_palindrome("Rotor"));
  println!("{}",is_palindrome("hello"));
  println!("{}",is_palindrome("world"));
}
