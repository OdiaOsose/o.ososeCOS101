use std::fs::remove_file;
use std::fs::File;
use std::io::Write;
fn main() {
    let mut file = File::create("data.txt").expect("file could not be created");
    file.write_all("Students are taught by teachers".as_bytes()).expect("write failed");
    println!("File created");
    remove_file("data.txt").expect("could not remove file");
    println!("File has been removed.");
}
