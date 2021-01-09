pub fn run() {
  let age: u8 = 18;
  let is_of_age = age >= 21; // if age >= 21 { true } else { false }
  let check_id: bool = false;

  if is_of_age && check_id {
    println!("What would you like to drink?")
  } else if !is_of_age && check_id {
    println!("Sorry, you have to leave")
  } else {
    println!("I'll need to see your ID")
  }
}