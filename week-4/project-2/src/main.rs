use std::io;

fn main() {
    println!("Hello! This is your Employee Annual Incentive Calculator!\n");

    // Input employee experience status
    let mut input1 = String::new();
    println!("Is the employee experienced? (yes/no):");
    io::stdin()
    .read_line(&mut input1)
    .expect("Failed to read input");
    let experience = input1.trim().to_lowercase();

    // Input employee age
    let mut input2 = String::new();
    println!("Please Enter employee age:");
    io::stdin()
    .read_line(&mut input2)
    .expect("Failed to read input");
    let age: u8 = input2.trim().parse().expect("Invalid input");

    // Calculate incentive
    if experience == "yes" || experience == "y" || experience == "YES" {
        if age >= 40 {
        println!("Congrats! Your incentive is N 1,560,000.0");
        } else if age >= 30 {
            println!("Congrats! Your incentive is N 1,480,000.0");
        } else if age < 28 {
            println!("Congrats! Your incentive is N 1,300,000.0");
        }
        else {
            // For ages 28-29 (not specified)
            println!("No specified incentive. Next time!");
        } 
    } else if experience == "no" || experience == "n" || experience == "NO" {
        println!("Your incentive is N 100,000.0");
    }
    else {
        println!("Your experience input is invalid, Please Try Again!")
    }
}
