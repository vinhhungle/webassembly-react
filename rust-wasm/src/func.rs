pub fn run() {
  println!("{}", greet("World"))
}

fn greet(name: &str) -> String {
  let mut out = String::from("Hello ");
  out.push_str(name);
  out
}