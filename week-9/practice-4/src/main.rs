use std::io::Write;
use std::fs::OpenOptions;

fn main() {
    let mut file = OpenOptions::new().append(true).open("../practice-1/src/data.txt")
    .expect("cannot open file");
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
    file.write_all("\nThis is the appendage to the document.".as_bytes())
    .expect("write failed");
    println!("Successfully appended the file.");
}
