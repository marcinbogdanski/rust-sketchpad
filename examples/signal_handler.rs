// Source: https://detegr.github.io/doc/ctrlc/

fn main() {
    println!("Hello signal handler!");

    let keep_running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = keep_running.clone();

    ctrlc::set_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    while keep_running.load(std::sync::atomic::Ordering::SeqCst) {}
    
    println!("Got it! Exiting...");
}

