use std::io::Read;
use std::io::Write;
use std::fs::File;

fn main() {
    let mut file = File::create("welcome_message.txt").unwrap();
    file.write_all("\nWelcome! We hope you enjoy your stay here."
        .as_bytes()).unwrap();
    println!("Data written to file");

    let mut file = File::open("welcome_message.txt")
    .expect("failed to open requested file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("conversion failed");
    print!("{}", contents);
}
