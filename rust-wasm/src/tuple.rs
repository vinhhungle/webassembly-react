pub fn run() {
  let person: (&str, &str, i8) = ("Hung", "Saigon", 43); // Tuple group values of different types. Max is 12 values

  println!("{} is from {} and is {}", person.0, person.1, person.2);
}