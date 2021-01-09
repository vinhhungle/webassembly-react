pub fn run() {
  let mut n = 0;

  loop {
    println!("Loop Number {}", n);
    n += 1;
    if n >= 20 { break };
  }

  n = 0;
  while n < 20 {
    println!("While Number {}", n);
    n += 1;
  }

  for n in 0..20 {
    println!("For Number {}", n);
  }
}