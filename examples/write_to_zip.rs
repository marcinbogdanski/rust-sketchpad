use std::fs::File;
use std::io::{BufWriter, Write};

fn main(){
    println!("Hello write to zip!");

    let file = File::create("output.zip").expect("create failed");
    let mut zip = zip::ZipWriter::new(file);
    zip.start_file("output.txt", zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)).expect("start_file failed");

    let lines = vec!["Line 1", "Line 2", "Line 3"];

    for line in lines {
        // Write the line to the file
        writeln!(zip, "{}", line).expect("write failed");
        panic!("write failed");
    }

    // Dropping the `ZipWriter` will have the same effect, but may silently fail
    zip.finish().expect("finish failed");

    println!("Done");

}
