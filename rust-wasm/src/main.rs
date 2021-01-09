mod vector;
// mod structs;
mod json;

fn main() {
  let result = json::run();
  match result {
    Ok(v) => println!("Result: {:#?}", v),
    Err(e) => println!("Error: {:?}", e)
  }
  // let _c = structs::RGB {
  //   red: 0, green: 0, blue: 0
  // };
  vector::nested::run();
}