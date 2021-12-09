use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // Spawn the thread with thread::spawn
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello from thread, number: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..3 {
        println!("Hello from main: {}", i);
        thread::sleep(Duration::from_millis(100));
    }

    // Handle.join() ensures that main waits for the thread to finish.
    handle.join().unwrap();

    // ////////////////////////////////////////
    // Message passing
    // ////////////////////////////////////////

    let (tx, rx) = mpsc::channel();

    // -----
    // ----- Simple example:
    // -----
    // Move allows the moving of ownership to the thread
    // thread::spawn(move || {
    //     let vals: Vec<String> = vec![
    //         String::from("am"),
    //         String::from("in"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         // send returns a Result
    //         // here .unwrap() is used, but in production should handle properly
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // -----
    // ----- Debugging; two threads, clone the tx.
    // -----

    // let tclone = tx.clone();

    // thread::spawn(move || {
    //     let vals: Vec<String> = vec![
    //         String::from(format!("T: {}: hello", 1)),
    //         String::from(format!("T: {}: there", 1)),
    //         String::from(format!("T: {}: anakin", 1)),
    //         String::from(format!("T: {}: I", 1)),
    //     ];

    //     for val in vals {
    //         tclone.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // thread::spawn(move || {
    //     let vals: Vec<String> = vec![
    //         String::from(format!("T: {}: have", 2)),
    //         String::from(format!("T: {}: the", 2)),
    //         String::from(format!("T: {}: higher", 2)),
    //         String::from(format!("T: {}: ground", 2)),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // -----
    // ----- Now looping through creating N threads
    // -----

    // Why does this not exit?
    // Is it because the original tx is never 'finished'?
    for i in 1..3 {
        // I think this is the problem.
        let t2 = tx.clone();
        thread::spawn(move || {
            let vals: Vec<String> = vec![
                String::from(format!("T: {}: hello", i.clone())),
                String::from(format!("T: {}: there", i.clone())),
                String::from(format!("T: {}: anakin", i.clone())),
                String::from(format!("T: {}: I", i.clone())),
                String::from(format!("T: {}: have", i.clone())),
                String::from(format!("T: {}: the", i.clone())),
                String::from(format!("T: {}: higher", i.clone())),
                String::from(format!("T: {}: ground", i.clone())),
            ];

            for val in vals {
                // send returns a Result
                // here .unwrap() is used, but in production should handle properly
                t2.clone().send(val).unwrap();
                thread::sleep(Duration::from_millis(100 * i.clone()));
            }
        });
    }

    // DROP is needed so that the original tx goes out of scope. :)
    // Now it doesn't hang! Cool.
    drop(tx);

    for received in rx {
        println!("Got value: {}", received);
    }

}
