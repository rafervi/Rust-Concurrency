use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

const NUM_THREADS:usize = 20;

fn start_thread(d:usize, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
       println!("setting timer {}", d);
        thread::sleep(Duration::from_secs(d as u64));
        println!("sending {}", d);
        tx.send(d).unwrap()
    });
}

fn main() {
    let mut threads = vec![];
    for i in 0..10{
        let th = thread::spawn( move ||{
            sleep(Duration::from_millis(i*1000));
            println!("new thread {}", i);
        } );
        threads.push(th);
    }
    for t in threads {
        t.join();
    }
    println!("main thread");

    let (tx,rx) = mpsc::channel();
    for i in 0..NUM_THREADS {
        start_thread(i, tx.clone());
    }
    for j in rx.iter().take(NUM_THREADS) {
        println!("received {}", j);
    }
    let c = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for i in 0..10 {
        let c = Arc::clone(&c);
        let t = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        });
        threads.push(t);
    }

    for th in threads {
        th.join().unwrap();
    }
    println!("Result {}", *c.lock().unwrap());//lock.lock() try.lock() lock.ispoisoned()


}
