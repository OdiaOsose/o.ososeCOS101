use std::io;
use std::fs::File;
use std::io::Read;

fn main() {
    loop {
    println!("Hello there, Welcome To GLOBACOM DATABASE VIEWER!");
    println!("Let's get started");

    let role = get_role();

    if role == "Q" || role == "q" || role == "6" {
        println!("Thank you for using GLOBACOM. Have a good day!");
        break;
    }
    display(&role);
    }
}

fn get_role() -> String {
    println!("What is your role?");
    println!("Enter a number (1-5):
              1 - Administrator
              2 - Project Manager
              3 - Employee
              4 - Customer
              5 - Vendor");
    println!("Enter 'Q' to quit");

    let mut choice = String::new();
    io::stdin()
    .read_line(&mut choice)
    .expect("Failed to read your role. Please Try Again.");
    choice.trim().to_string()
}

fn display(role: &str) {
    match role {
        "1" => admin_view(),
        "2" => proj_manager_view(),
        "3" => employee_view(),
        "4" => customer_view(),
        "5" => vendor_view(),
        _ => println!("Invalid input. Please enter a number 1-5"),
    }
}

fn admin_view() {
    let mut file = File::open("globacom_db.sql").expect("Cannot open the file.");
    let mut admin_contents = String::new();
    file.read_to_string(&mut admin_contents).expect("Cannot read the file.");
    println!("{}",admin_contents);

}

fn proj_manager_view() {
    let mut file = File::open("project_tb.sql").expect("Cannot open the file.");
    let mut project_contents = String::new();
    file.read_to_string(&mut project_contents).expect("Cannot read the file.");
    println!("{}",project_contents);
}

fn employee_view() {
    let mut file = File::open("staff_tb.sql").expect("Cannot open the file.");
    let mut staff_contents = String::new();
    file.read_to_string(&mut staff_contents).expect("Cannot read the file.");
    println!("{}",staff_contents);
}

fn customer_view() {
    let mut file = File::open("customers_tb.sql").expect("Cannot open the file.");
    let mut customers_contents = String::new();
    file.read_to_string(&mut customers_contents).expect("Cannot read the file.");
    println!("{}",customers_contents);
}

fn vendor_view() {
    let mut file = File::open("dataplans_tb.sql").expect("Cannot open the file.");
    let mut dataplan_contents = String::new();
    file.read_to_string(&mut dataplan_contents).expect("Cannot read the file.");
    println!("{}",dataplan_contents);
}