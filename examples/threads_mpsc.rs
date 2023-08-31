use std::time::Duration;
use std::sync::mpsc;
use std::thread;


fn producer(name: &str, num: i64, tx: mpsc::Sender<String>) {
    for i in 0..num {
        let msg = format!("{}", i);
        println!("Producer {} sending: {}", name, msg);
        tx.send(msg).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}

fn consumer(name: &str, rx: mpsc::Receiver<String>) {
    loop {
        match rx.recv() {
            Ok(msg) => println!("Consumer {} got: {}", name, msg),
            Err(e) => {
                println!("Consumer {} got error: {}", name, e);
                break;
            }
        }
    }
}

fn main() {
    println!("Hello MPSC!");

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    let thread_1 = thread::spawn(move || producer("prod1", 5, tx));
    let thread_2 = thread::spawn(move || producer("prod2", 7, tx1));
    let thread_3 = thread::spawn(move || consumer("cons1", rx));

    thread_1.join().unwrap();
    thread_2.join().unwrap();
    thread_3.join().unwrap();
}
