extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Вгадайте число!");
    let secret_number = rand::thread_rng().gen_range(1, 100);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .ok()
        .expect("Something wend wrong");

    let guess: u32 = guess.trim().parse().ok().expect("Please enter a number");

    println!("Your number is {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Your number is less than secret"),
        Ordering::Greater => println!("Your number is great than secret"),
        Ordering::Equal => println!("Sucess!!!!"),
    }
}
