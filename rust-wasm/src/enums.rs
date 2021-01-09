enum Movement {
  Up,
  Down,
  Left,
  Right
}

fn move_it(m: Movement) {
  match m {
    Movement::Up => println!("Move Up"),
    Movement::Down => println!("Move Down"),
    Movement::Left => println!("Move Left"),
    Movement::Right => println!("Move Right"),
  }
}

pub fn run() {
  move_it(Movement::Up);
  move_it(Movement::Down);
  move_it(Movement::Left);
  move_it(Movement::Right);
}