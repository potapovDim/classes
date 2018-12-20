use std::{thread, time};
use std::sync::{Arc, Mutex};

struct Table {
    forks: Vec<Mutex<()>>,
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, rigth: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: rigth,
        }
    }
    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} start eat", self.name);
        // sleep static method argument should be
        // result of time duration from millis method
        thread::sleep(time::Duration::from_millis(1500));
        println!("{} finish eat", self.name);
    }
}
fn main() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    let philosophes = vec![
        Philosopher::new("First philosopher", 0, 1),
        Philosopher::new("Second philosopher", 1, 2),
        Philosopher::new("Third philosopher", 2, 3),
        Philosopher::new("Fourth philosopher", 3, 4),
        Philosopher::new("Fifth philosopher", 4, 5),
    ];
    let handless: Vec<_> = philosophes
        .into_iter()
        .map(|p| {
            let table = table.clone();
            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();

    for h in handless {
        h.join().unwrap();
    }
}
