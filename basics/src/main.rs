fn _is_divisible_by(or_num: i32, divisible: i32) -> bool {
  if or_num == 0 {
    return false;
  }
  or_num % divisible == 0
}

fn _fizzbuzz_question(num_fb: i32) {
  for num in 1..num_fb + 1 {
    let (by_three, by_five) = (_is_divisible_by(num, 3), _is_divisible_by(num, 5));
    if by_five && by_three {
      println!("fizzbuzz");
    } else if by_three {
      println!("fizz");
    } else if by_five {
      println!("buzz");
    } else {
      println!("qua qua qua");
    }
  }
}

fn main() {}
// in fn does not return anything it returns () by default
// fn a() -> () {
//   println!("Return ()");
// }
// fn b() {
//   println!("return ()");
// }
