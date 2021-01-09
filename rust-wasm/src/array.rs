pub fn run() {
  let mut numbers: [i32; 5] = [1,2,3,4,5]; // Array is fixed list where elements are the same data type
  numbers[1] = 20;

  println!("numbers {:?} occupies {} bytes", numbers, std::mem::size_of_val(&numbers));

  let slice : &[i32] = &numbers[0..3];
  println!("Slice: {:?}", slice);
}