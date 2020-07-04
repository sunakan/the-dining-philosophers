use std::thread;
use std::sync::{Mutex, Arc};

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
}

impl Philosopher {
    fn eat(&self, table: &Table) {
        println!("{} は左にあるフォーク{}を...", self.name, self.left);
        let _left = table.forks[self.left].lock().unwrap();
        println!("{} は左にあるフォーク{}を手に取った", self.name, self.left);
        thread::sleep_ms(5000);
        let _right = table.forks[self.right].lock().unwrap();
        println!("{} は右にあるフォーク{}を手に取った", self.name, self.right);
        println!("{} は食べている", self.name);
        thread::sleep_ms(1000);
        println!("{} は食べ終わったので、フォーク{},{}をテーブルに置く", self.name, self.left, self.right);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}
impl Table {
    fn new() -> Table {
        Table {
            forks: vec![
                Mutex::new(()),
                Mutex::new(()),
                Mutex::new(()),
                Mutex::new(()),
                Mutex::new(()),
            ],
        }
    }
}

fn main() {
    let table = Arc::new(Table::new());
    let philosophers = get_philosophers();
    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            thread::sleep_ms(500);
            let table = table.clone();
            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}

// Ellenがフォークを取る順番を4,0とすると、Daveがずっと取れない
// Daveが食事できないとCarolもフォーク3が無いので、無理
// Carolが無理だと、Bobも無理
// Bobが無理だと、Aliceも無理
// Aliceが無理だとElllenも無理
// Ellenが無理だと、Daveも無理...繰り返し => DeadLock
fn get_philosophers() -> Vec<Philosopher> {
    vec![
        Philosopher::new("哲学者1-Alice", 0, 1),
        Philosopher::new("哲学者2-Bob", 1, 2),
        Philosopher::new("哲学者3-Carol", 2, 3),
        Philosopher::new("哲学者4-Dave", 3, 4),
        //Philosopher::new("哲学者5-Ellen", 4, 0),
        Philosopher::new("哲学者5-Ellen", 0, 4),
    ]
}
