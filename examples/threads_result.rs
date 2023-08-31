use std::thread;
use std::error::Error;
use std::time::Duration;
use std::any::Any;

fn thread_function(x: i64, y: i64) -> Result<i64, Box<dyn Error + Send + Sync>> {
    thread::sleep(Duration::from_secs(1));
    match x {
        0 => return Err("x is zero".into()),
        1 => panic!("x is one"),
        _ => return Ok(y + x),
    }
}

fn main() {
    println!("Hello threads simple!");
    
    let handle = thread::Builder::new().name("thread1".to_string()).spawn(move || {
        thread_function(1, 20)
    }).unwrap();

    let result: Result<Result<i64, Box<dyn Error + Send + Sync>>, Box<dyn Any + Send>> = handle.join();
    println!("-------------------");
    match result {
        Ok(res) => {
            match res {
                Ok(x) => println!("Result Value {}", x),
                Err(e) => println!("Result Error: {}", e),
            }
        },
        Err(e) => {
            match e.downcast_ref::<&str>() {
                Some(s) => println!("Thread CRASHED: {}", s),
                None => println!("What happened?"),
            }
        }
    }
    println!("-------------------");

    

}


// fn start_server() -> Result<(), Box<dyn std::error::Error>> {
//     let x = std::thread::spawn(move || -> Result<(), std::io::Error> {
//         run_server()?;
//         Ok(())
//     });

//     x.join().unwrap()?; // Now you can throw your error up because you joined your thread.
//                         // ...
//     Ok(())
// }

// fn run_server() -> Result<(), std::io::Error> {
//     Err(std::io::Error::new(std::io::ErrorKind::Other, "hello"))
// }

// fn main() {
//     let x = start_server();
//     println!("{:?}", x);
// }