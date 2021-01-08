pub fn run() {
  let x = 1; // i32
  let y = 1.0; // f64;
  let z: f32 = 2.1; // explicit
  let s = "Hello "; // &str;
  let c = 'c'; // char
  let b = false; // bool
  let face = '\u{1F600}';

  println!("====== types ======");

  println!("x: {}, y: {}, {:?}", x, y, (z,c,b,face,s,s.len()));

  let mut hello = String::from(s);
  hello.push('W');
  hello.push_str("orld");

  println!("{:?}", hello);
  println!("{:?}", hello.is_empty());
  println!("{:?}", hello.capacity());

  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  let mut str = String::with_capacity(10);
  str.push_str("Ey");

  assert_eq!(3, str.len());
  
  // println!("Max i8: {}", std::i8::MAX);
  // println!("Max u8: {}", std::u8::MAX);

  // println!("Max 16: {}", std::i16::MAX);
  // println!("Max i32: {}", std::i32::MAX);
  // println!("Max i64: {}", std::i64::MAX);

  // println!("Max f32: {}", std::f32::MAX);
  // println!("Max f64: {}", std::f64::MAX);
}