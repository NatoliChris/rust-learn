use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    for _ in 0..10 {
        let ctr = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = ctr.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Res: {}", *counter.lock().unwrap());
}
