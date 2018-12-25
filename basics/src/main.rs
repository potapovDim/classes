fn _fizzbuzz_question(num_fb: i32) {
  for num in 1..num_fb + 1 {
    if num % 3 == 0 && num % 5 == 0 {
      println!("fizzbuzz");
    } else if num % 3 == 0 {
      println!("fizz");
    } else if num % 5 == 0 {
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
