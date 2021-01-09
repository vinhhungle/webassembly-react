#![allow(dead_code)]
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use pulldown_cmark::{Parser, Options, html};

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

mod structs;
use structs::Person;

#[derive(Serialize, Deserialize)]
pub struct Example {
    pub field1: HashMap<u32, String>,
    pub field2: Vec<Vec<f32>>,
    pub field3: [f32; 4],
    pub field4: Vec<Person>,
}

#[wasm_bindgen]
pub fn get_json() -> JsValue {
    let mut field1 = HashMap::new();
    field1.insert(0, String::from("ex"));
    
    let example = Example {
        field1,
        field2: vec![vec![1., 2.], vec![3., 4.]],
        field3: [1., 2., 3., 4.],
        field4: vec![Person::new("Hung", "Le"), Person::new("Vy", "Le")]
    };

    JsValue::from_serde(&example).unwrap()
}

#[wasm_bindgen]
pub fn send_json(val: &JsValue) -> JsValue {
    let example: Example = val.into_serde().unwrap();
    
    JsValue::from_serde(&example).unwrap()
}

#[wasm_bindgen]
pub fn add(a: f32, b: f32) -> f32 {
  a + b
}

#[wasm_bindgen]
pub fn fibonacci(n: f32) -> f32 {
  let mut a: f32 = 0.0;
  let mut b: f32 = 1.0;
  for _ in 0..n.round() as i32 {
      let tmp = b;
      b += a;
      a = tmp;
  }
  return a;
}

#[wasm_bindgen]
pub fn bm_fibonacci(n: f32, m: f32) {
  for _ in 0..m.round() as i64 {
    fibonacci(n);
  }
}

#[wasm_bindgen]
pub fn render_md(markdown_input: &str) -> String {
  // Set up options and parser. Strikethroughs are not part of the CommonMark standard
  // and we therefore must enable it explicitly.
  let mut options = Options::empty();
  options.insert(Options::ENABLE_STRIKETHROUGH);
  let parser = Parser::new_ext(markdown_input, options);

  // Write to String buffer.
  let mut html_output = String::new();
  html::push_html(&mut html_output, parser);

  return html_output;
}

pub fn run() {
  println!("====== lib ======");

  let markdown_input = "Hello world, this is a ~~complicated~~ *very simple* example.";
  let expected_html = "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";

  let html_output = render_md(markdown_input);
  assert_eq!(expected_html, &html_output);
  
  println!("Fib: {}", fibonacci(45.0));
}