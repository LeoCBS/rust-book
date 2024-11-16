use std::{
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread count {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });
    handle.join().unwrap();
    for i in 0..10 {
        println!("count {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let val = vec![
            String::from("hi"),
            String::from("from"),
            String::from("thread"),
        ];
        for v in val {
            tx.send(v).unwrap();
        }
    });

    thread::spawn(move || {
        let val = vec![
            String::from("hi2"),
            String::from("from2"),
            String::from("thread2"),
        ];
        for v in val {
            tx1.send(v).unwrap();
        }
    });

    for r in rx {
        println!("got {r}");
    }

    //we have atomic options instead use Mutex<T> like https://doc.rust-lang.org/std/sync/atomic/index.html
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        //Fortunately, Arc<T> is a type like Rc<T> that is safe to use in concurrent situations. The a stands for atomic, meaning it’s an atomically reference counted type.
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
