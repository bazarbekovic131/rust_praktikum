use std::thread;
use std::time::Duration;
use std::sync::{Arc, mpsc, Mutex};

fn main() {
    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     // for i in 1..10 {
    //     //     println!("hi number {} from the spawned thread!", i);
    //     //     thread::sleep(Duration::from_millis(1));
    //     // }

    //     println!("Here's a vector v: {:?}", v);
    // });
    // handle.join().unwrap();

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    for received in rx {
        println!("Got: {}",received);
    }

    let m = Mutex::new(5);

    {
        let mut n = m.lock().unwrap();
        *n = 6;
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}