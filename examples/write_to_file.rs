use std::fs::File;
use std::io::{BufWriter, Write};

fn main(){
    println!("Hello write to file!");

    let file = File::create("output.txt").expect("create failed");
    let mut writer = BufWriter::new(file);

    let lines = vec!["Line 1", "Line 2", "Line 3"];

    for line in lines {
        // Write the line to the file
        writeln!(writer, "{}", line).expect("write failed");
    }

    // Make sure to flush the buffer to ensure all data is written to the file
    writer.flush().expect("flush failed");

    println!("Done");

}
