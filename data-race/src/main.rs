use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let handles: Vec<_> = (0..3)
        .map(|i| {
            let data = data.clone();
            thread::spawn(move || {
                let mut data = data.lock().unwrap();
                data[i] += 1;
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", data);
}
