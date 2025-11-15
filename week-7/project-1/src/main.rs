use std::io;

fn main() {
    println!("Hello! Welcome to Geometric Calculator!");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");
    println!("What would you like to calculate? (1, 2, 3, 4 or 5): ");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read");
    let choice: u8 = choice.trim().parse().expect("Enter a number");
    
    if choice == 1 {
        trapezium();
    } else if choice == 2 {
        rhombus();
    } else if choice == 3 {
        parallelogram();
    } else if choice == 4 {
        cube();
    } else if choice == 5 {
        cylinder();
    } else {
        println!("Invalid choice, please pick a number from 1-5");
    }
}

fn trapezium() {
    println!("Enter height: ");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed");
    let height: f64 = height.trim().parse().expect("Enter number");
    
    println!("Enter base1: ");
    let mut base1 = String::new();
    io::stdin().read_line(&mut base1).expect("Failed");
    let base1: f64 = base1.trim().parse().expect("Enter number");
    
    println!("Enter base2: ");
    let mut base2 = String::new();
    io::stdin().read_line(&mut base2).expect("Failed");
    let base2: f64 = base2.trim().parse().expect("Enter number");
    
    let area = height / 2.0 * (base1 + base2);
    println!("Area of Trapezium = {}", area);
}

fn rhombus() {
    println!("Enter diagonal 1: ");
    let mut d1 = String::new();
    io::stdin().read_line(&mut d1).expect("Failed");
    let d1: f64 = d1.trim().parse().expect("Enter number");
    
    println!("Enter diagonal 2: ");
    let mut d2 = String::new();
    io::stdin().read_line(&mut d2).expect("Failed");
    let d2: f64 = d2.trim().parse().expect("Enter number");
    
    let area = 0.5 * d1 * d2;
    println!("Area of Rhombus = {}", area);
}

fn parallelogram() {
    println!("Enter base: ");
    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("Failed");
    let base: f64 = base.trim().parse().expect("Enter number");
    
    println!("Enter altitude: ");
    let mut altitude = String::new();
    io::stdin().read_line(&mut altitude).expect("Failed");
    let altitude: f64 = altitude.trim().parse().expect("Enter number");
    
    let area = base * altitude;
    println!("Area of Parallelogram = {}", area);
}

fn cube() {
    println!("Enter side length: ");
    let mut side = String::new();
    io::stdin().read_line(&mut side).expect("Failed");
    let side: f64 = side.trim().parse().expect("Enter number");
    
    let area = 6.0 * side * side;
    println!("Area of Cube = {}", area);
}

fn cylinder() {
    println!("Enter radius: ");
    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("Failed");
    let radius: f64 = radius.trim().parse().expect("Enter number");
    
    println!("Enter height: ");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed");
    let height: f64 = height.trim().parse().expect("Enter number");
    
    let pi = 3.141592654;
    let volume = pi * radius * radius * height;
    println!("Volume of Cylinder = {}", volume);
}