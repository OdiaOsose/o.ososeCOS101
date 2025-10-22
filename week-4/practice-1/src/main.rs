//Rust program to output name and age

use std::io;
fn main() {
    println!("\n Hello! This is the Student Information Management System!");

//input name
println!("\nPlease Enter your name.");
let mut name = String::new();
io::stdin()
.read_line(&mut name)
.expect("Failed to read input");
println!("Your name is: {}", name);

//input age
<<<<<<< HEAD
println!("\nPlease Enter your age.");
=======
println!("\n Please Enter your age.");
>>>>>>> d6af629b7a40724e32fd4c54b5211b65acc4a00c
let mut age = String::new();
io::stdin().read_line(&mut age).expect("Failed to read input");
let age:u8 = age.trim().parse().expect("Input not an integer");
println!("Your age is: {}", age);
<<<<<<< HEAD
}
=======
}
>>>>>>> d6af629b7a40724e32fd4c54b5211b65acc4a00c
