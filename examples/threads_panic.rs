
use std::time::Duration;
use std::thread;

fn producer() {
    for i in 0..7 {                                // TEST: change to 17 to test hanging thread
        println!("  Producer1 sending: {}", i);
        thread::sleep(Duration::from_secs(1));
    }
}

fn producer2() {
    for i in 0..7 {
        println!("  Producer2 sending: {}", i);
        thread::sleep(Duration::from_secs(1));

        if i == 2 {
            panic!("  Producer2 panicking!");
        }
    }
}

fn main() {
    println!("Hello threads panic!");

    let thread_1 = thread::Builder::new().name("thread1".to_string()).spawn(move || producer()).unwrap();
    let thread_2 = thread::Builder::new().name("thread2".to_string()).spawn(move || producer2()).unwrap();

    // Monitor for any thread to finish or crash
    loop {
        if thread_1.is_finished() || thread_2.is_finished() {
            if thread_1.is_finished() {
                println!("Thread 1 finished first");
            }
            if thread_2.is_finished() {
                println!("Thread 2 finished first");
            }
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }

    // After first thread finishes, wait for the other thread to finish
    for _ in 0..5 {
        if thread_1.is_finished() && thread_2.is_finished() {
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }

    // If both threads are finished, join them
    if thread_1.is_finished() && thread_2.is_finished() {
        println!("-------------------");
        match thread_1.join() {
            Ok(_) => println!("Thread 1 finished ok"),
            Err(_) => println!("Thread 1 CRASHED"),
        }
        match thread_2.join() {
            Ok(_) => println!("Thread 2 finished ok"),
            Err(_) => println!("Thread 2 CRASHED"),
        }
        println!("-------------------");
    } else {
        panic!("Threads did not finish")
    }

    
   
    println!("Done");
}
