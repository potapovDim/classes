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
    // let one_to_one_hundred = (0..100).collect::<Vec<i32>>();
    let greater_than_two = (0..10).find(|x| *x>2);
    // fold -> js reduce
    for i in &greater_than_two {
        println!("{} greater ", i);
    }
    let sum = (0..20).fold(0, |acc, item| acc + item);

    println!("{}", sum);
}
