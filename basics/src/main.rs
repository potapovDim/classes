// * for unnaming
// & ref ref mut for destruction
fn main() {
  // _reference_example()
  _if_let_example();
}
// if le provide posibility of assertion by destruction
fn _if_let_example() {
  #[derive(Debug)]
  enum IfLetExampe {
    _Foo,
    Bar,
    _Baz,
  }

  let from_enum = IfLetExampe::Bar;
  // it is not defininng, it is assertions
  if let IfLetExampe::Bar = from_enum {
    println!("From enum example worked as expected");
  }
}
// references
fn _reference_example() {
  let ref_num = &4;
  match ref_num {
    &_val => println!("by reference"),
    _ => println!("TEST"),
  }

  match *ref_num {
    _val => println!("By unnaming"),
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

  let _a = match boolean {
    false => 0,
    true => 1,
  };

  println!("a value should be binnary 0 or 1  = {}", _a);
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
    Colors::RGBA(_x, _y, z, _a) => println!("RGBA WAS DEFINED {}", z),
    _ => println!("Everything else "),
  }
}
