use chrono::Utc;

fn main(){
    println!("Hello datetime span!");

    let start_time = Utc::now().time();
    let end_time = Utc::now().time();
    let diff = end_time - start_time;
    println!("Total time taken to run is {}", diff.num_minutes());

    println!("Waiting for 5 seconds...");
    let mut next_time = Utc::now().time() + chrono::Duration::seconds(5);
    while Utc::now().time() < start_time + chrono::Duration::seconds(16) {
        if Utc::now().time() > next_time {
            println!("5 seconds passed");
            next_time = Utc::now().time() + chrono::Duration::seconds(5);
        }
    }

}
