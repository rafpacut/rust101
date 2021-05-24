use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn simple_communication() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("thread!"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn mutex_demo() {
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

    println!("num is: {}", counter.lock().unwrap());
}

fn main() {
    mutex_demo();
}
