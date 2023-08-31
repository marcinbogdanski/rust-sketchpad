// use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{TimeZone, Utc};

fn main() {
    println!("Hello datetime UTC!");

    // Using system time
    // match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
    //     Ok(n) => {
    //         let nanos = n.as_secs() * 1_000_000_000 + n.subsec_nanos() as u64;
    //         println!("{} nanoseconds since the UNIX epoch", nanos);
    //     }
    //     Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    // }
    
    // Using chrono
    // 2023-08-08_09-38-22_015784484_UTC
    let datetime = Utc::now();
    let timestamp: i64 = datetime.timestamp_nanos();
    println!("{} nanoseconds since the UNIX epoch", timestamp);
    println!("{}_{:09}_UTC",
        datetime.format("%Y-%m-%d_%H-%M-%S"),
        datetime.timestamp_subsec_nanos());

    // Using chrono, double check
    // 2023-08-08_09-38-22_015784484_UTC
    let datetime_2 = Utc.timestamp_nanos(timestamp);
    let nanos = timestamp % 1_000_000_000;
    println!("{}_{:09}_UTC", datetime_2.format("%Y-%m-%d_%H-%M-%S"), nanos);
    
    // println!("'{}'", datetime.format("%Y-%m-%d_%H-%M-%S_%N_UTC"));
    println!("Got it! Exiting...");
    
}
