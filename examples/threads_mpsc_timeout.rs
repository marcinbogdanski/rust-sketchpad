use std::time::Duration;
use std::sync::mpsc;
use std::thread;


fn producer(name: &str, num: i64, tx: mpsc::Sender<String>) {
    for i in 0..num {
        let msg = format!("{}", i);
        println!("Producer {} sending: {}", name, msg);
        tx.send(msg).unwrap();
        thread::sleep(Duration::from_secs(3));
    }
}

fn consumer(name: &str, rx: mpsc::Receiver<String>) {
    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(msg) => println!("Consumer {} got: {}", name, msg),
            Err(e) => {
                match e {
                    mpsc::RecvTimeoutError::Timeout => {
                        println!("Consumer {} timed out", name);
                        continue;
                    },
                    mpsc::RecvTimeoutError::Disconnected => {
                        println!("Consumer {} disconnected", name);
                        break;
                    },
                }
            }
        }
    }
}

fn main() {
    println!("Hello MPSC timeout!");

    let (tx, rx) = mpsc::channel();

     let thread_1 = thread::spawn(move || producer("prod1", 5, tx));
    let thread_3 = thread::spawn(move || consumer("cons1", rx));

    thread_1.join().unwrap();
    thread_3.join().unwrap();
}
