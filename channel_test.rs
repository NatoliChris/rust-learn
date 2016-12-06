use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 10;

fn main() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for i in 0..NTHREADS {
        // Clone sender for each thread
        let thead_tx = tx.clone();

        thread::spawn(move || {
            //Each thread will send a message on the channel
            thead_tx.send(i).unwrap();
            println!("Thread #{} DONE", i);
        });
    }

    let mut thread_ids = Vec::with_capacity(NTHREADS as usize);

    for _ in 0..NTHREADS {
        thread_ids.push(rx.recv());
    }

    println!("{:?}", thread_ids);

}
