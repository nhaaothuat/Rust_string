fn exercise1(color: &str) -> String{
let result =  match color{
    "white" =>String::from("white"),
    _ => String::from("No one at all"),    
};
  return result;
 }

fn main(){
  let color = "white";
  println!("{}",exercise1(color));
}
