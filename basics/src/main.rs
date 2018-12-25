// * for unnaming
// & ref ref mut for destruction
fn main() {
  _reference_example()
}
fn _reference_example() {
  let ref_num = &4;
  match ref_num {
    &val => println!("by reference"),
    _ => println!("TEST"),
  }

  match *ref_num {
    val => println!("By unnaming"),
  }
}

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

fn _basic_enum_example() {
  #[derive(Debug)]
  enum Colors {
    Red,
    Green,
    Blue,
    RGBA(i32, i32, i32, f32),
  }
  let color = Colors::RGBA(3, 4, 5, 0.1);
  match color {
    Colors::RGBA(x, y, z, a) => println!("RGBA WAS DEFINED"),
    _ => println!("Everything else "),
  }
}
