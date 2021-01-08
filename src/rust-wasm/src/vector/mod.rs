pub mod nested;

#[allow(dead_code)]
pub fn run() {
  let mut numbers: Vec<i32> = vec![1,2,3,4,5]; // Array is unfixed list where elements are the same data type
  numbers[1] = 20;
  numbers.push(6);

  println!("numbers {:?} occupies {} bytes", numbers, std::mem::size_of_val(&numbers));

  let slice : &[i32] = &numbers[0..3];
  println!("Slice: {:?}", slice);

  for item in numbers.iter_mut() { // similar to Array.map
    *item *= 2;
  }

  for item in numbers.iter() {
    println!("Number {}", item)
  }
}