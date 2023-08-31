use clap::{Arg, Command};


fn main() {
    println!("Hello arguments!");

    let matches = Command::new("My Test Program")
        .version("0.1.0")
        .author("Mr Awesome")
        .about("Teaches argument parsing")
        .arg(Arg::new("prod")
                 .short('p')
                 .long("prod")
                 .required(false)
                 .num_args(0)
                 .help("Are we in production?"))
        .get_matches();

    if matches.get_flag("prod") {
        println!("We are in production mode!");
    } else {
        println!("We are in development mode!");
    }

    println!("Done");
}
