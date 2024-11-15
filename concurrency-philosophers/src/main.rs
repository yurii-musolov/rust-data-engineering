// https://www.coursera.org/learn/data-engineering-rust/supplement/uzElu/lesson-reflection

use rayon::prelude::*;
use std::thread;
use std::time::Duration;
use std::{
    fmt::Display,
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex, MutexGuard,
    },
};

const PHILOSOPHER_COUNT: usize = 10;
const CHOPSTICKS_COUNT: usize = 5;

fn main() {
    let (tx, rx) = mpsc::channel();

    let counter = Arc::new(Mutex::new(0_usize));

    let chopsticks: Vec<_> = (0..CHOPSTICKS_COUNT)
        .into_iter()
        .map(|id| Arc::new(Chopstick::new(id)))
        .collect();

    (0..PHILOSOPHER_COUNT)
        .map(|id| {
            let chopstick = match chopsticks.get(id % chopsticks.len()) {
                Some(chopstick) => chopstick.clone(),
                None => panic!("Reason."),
            };
            Philosopher::new(id, chopstick)
        })
        .for_each(|philosopher| {
            let tx = tx.clone();
            thread::spawn(move || philosopher.eat(tx));
        });

    loop {
        match rx.recv() {
            Ok(_id) => {
                let mut n = counter.lock().unwrap();
                *n += 1;
                if *n >= PHILOSOPHER_COUNT {
                    break;
                }
            }
            Err(_) => break,
        };
    }

    println!(
        "{} philosophers have finished eating",
        counter.lock().unwrap()
    );
}

struct Philosopher<T> {
    id: T,
    chopstick: Arc<Chopstick<T>>,
}

impl<T> Philosopher<T>
where
    T: Display + Copy,
{
    fn new(id: T, chopstick: Arc<Chopstick<T>>) -> Self {
        Self { id, chopstick }
    }
    fn eat(&self, tx: Sender<T>) {
        let _guard = self.chopstick.lock();
        println!(
            "Philosopher '{}' is eating.\t\tUse chopstick '{}'.",
            self.id,
            self.chopstick.get_id()
        );
        thread::sleep(Duration::from_secs(1));
        println!(
            "Philosopher '{}' finished eating.\tUnused chopstick '{}'.",
            self.id,
            self.chopstick.get_id()
        );
        match tx.send(self.id) {
            Ok(()) => {}
            Err(err) => println!("{}", err),
        };
    }
}

struct Chopstick<T> {
    id: T,
    mutex: Mutex<()>,
}

impl<T> Chopstick<T>
where
    T: Display + Copy,
{
    fn new(id: T) -> Self {
        Self {
            id,
            mutex: Mutex::new(()),
        }
    }
    fn get_id(&self) -> T {
        self.id
    }
    fn lock(&self) -> MutexGuard<'_, ()> {
        self.mutex.lock().unwrap()
    }
}
