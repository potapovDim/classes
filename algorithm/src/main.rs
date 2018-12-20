fn main() {
    let mut range = 0..10;
    // iterators
    loop {
        match range.next() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }
    // consumers
    let one_to_one_hundred = (0..100).collect::<Vec<i32>>();
}
