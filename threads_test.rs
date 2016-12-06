use std::thread;

static NTHREADS: i32 = 10;

fn main() {
    let mut children = vec![];

    for i in 0..NTHREADS {
        //Spawn a thread
        children.push(thread::spawn(move || {
            println!("Thread #{}", i);
        }));
    }

    for e in children {
        // Wait for thread finish and join
        let _ = e.join();
    }
}
