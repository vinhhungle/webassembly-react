
use serde::{Deserialize, Serialize};

// Traditional Struct
pub struct RGB {
  pub red: u8,
  pub green: u8,
  pub blue: u8
}

// Tuple Struct
pub struct Color(u8, u8, u8);

#[derive(Serialize, Deserialize)]
pub struct Person {
  pub first_name: String,
  pub last_name: String
}

impl Person {
  pub fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string()
    }
  }

  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }

  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {
  let mut c = RGB {
    red: 255, green: 0, blue: 0
  };
  c.green = 100;
  c.blue = 50;
  println!("RGB {} {} {}", c.red, c.green, c.blue);

  let c1 = Color(255, 0, 0);
  println!("Color.0 {}", c1.0);

  let mut p = Person::new("Hung", "Le");
  p.set_last_name("Lee");
  println!("{}", p.full_name());
  println!("{:?}", p.to_tuple());
}