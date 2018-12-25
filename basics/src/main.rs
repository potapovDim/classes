fn _example_1() {
  let munber = 12;
  match munber {
    1 => println!("1"),
    2 | 3 | 4 => println!("2,3,4"),
    // _ this for last numbers what are not included in existing items
    _ => println!("not a munber"),
  }

  let boolean = true;

  let a = match boolean {
    false => 0,
    true => 1,
  };
}

fn _destruction_math_example() {
  let kort = (1, 2);
  match kort {
    (_x, 2) => println!("x,2"),
    (1, _y) => println!("1,y"),
    _ => println!("Test"),
  }
}
fn main() {
}
