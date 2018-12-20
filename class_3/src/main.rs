use std::thread;

#[no_mangle]
pub extern fn process() {
    let handless: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(|| {
                let mut x = 0;
                println!("1");
                for _ in (0..5_000_000) {
                    x += 1
                }
                x
            })
        })
        .collect();
    for h in handless {
        println!(
            "thread finished with count {}",
            h.join().map_err(|_| "Could not join a thread").unwrap()
        );
    }
}
