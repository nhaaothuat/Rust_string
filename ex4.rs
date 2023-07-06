fn reverse_string(input: &str) -> String {
    let mut reversed = String::new();

    for c in input.chars().rev() {
        reversed.push(c);
    }

    reversed
}

fn main(){
   println!("{}",reverse_string("hello"));
   println!("{}",reverse_string("rust"));
   println!("{}",reverse_string("world"));
   println!("{}",reverse_string(""));
 }
