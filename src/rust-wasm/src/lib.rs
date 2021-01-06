#![allow(dead_code)]

use wasm_bindgen::prelude::*;
use pulldown_cmark::{Parser, Options, html};

#[wasm_bindgen]
pub fn add(a: f64, b: f64) -> f64 {
  a + b
}

#[wasm_bindgen]
pub fn fibonacci(n: f64) -> f64 {
  let mut a: f64 = 0.0;
  let mut b: f64 = 1.0;
  for _ in 0..n.round() as i64 {
      let tmp = b;
      b += a;
      a = tmp;
  }
  return a;
}

#[wasm_bindgen]
pub fn bm_fibonacci(n: f64, m: f64) {
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