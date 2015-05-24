use std::thread;
use std::sync::{Mutex, Arc};

struct Table {
    forks: Vec<Mutex<()>>,
}


struct Philosopher {
	name: String,
	left: usize,
    right: usize,
}

impl Philosopher {
	fn new(name: &str, left: usize, right: usize) -> Philosopher {
		Philosopher {
			name: name.to_string(),
			left: left,
			right: right,
		}
	}
	fn eat(&self, table: &Table) {
		let _left = table.forks[self.left].lock().unwrap();
		let _right = table.forks[self.right].lock().unwrap();
		println!("{} is eating", self.name);
		thread::sleep_ms(1000);
		println!("{} is done eating", self.name);
	}
}


fn main() {
	let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
	    Philosopher::new("Rousseau", 0, 1),
	    Philosopher::new("Voltaire", 1, 2),
	    Philosopher::new("Nietsche", 2, 3),
	    Philosopher::new("Plato", 3, 4),
	    Philosopher::new("Socrates", 0, 4),
    ];
    let handles : Vec<_> = philosophers.into_iter().map(|p| {
    	let table = table.clone();
    	thread::spawn(move || {
    		p.eat(&table);
    	})
    }).collect();

    for h in handles {
    	h.join().unwrap();
    }
}
