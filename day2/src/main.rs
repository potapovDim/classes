use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

fn find_str_offset(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    return None;
}

fn example1() {
    let data = Arc::new(Mutex::new(vec![1u32, 2, 3]));
    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        });
    }
    thread::sleep_ms(200)
}

fn example2() {
    let data = Arc::new(Mutex::new(0u32));
    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;

            tx.send(());
        });
    }

    for _ in 0..10 {
        rx.recv();
    }
}
fn main() {
    let file_name = "test_file.xlsx";
    match find_str_offset(file_name, '.') {
        Some(i) => println!("{}", &file_name[i + 1..]),
        None => println!("Not found"),
    }
}
