use std::io;

fn main() {
    println!("Hello! This is your Quadratic Equation Solver!");
    println!("For equation: ax² + bx + c = 0\n");

    // Input value a
    let mut input1 = String::new();
    println!("Please Enter value a:");
    io::stdin().read_line(&mut input1).expect("Failed to read input, sorry");
    let a: f64 = input1.trim().parse().expect("Invalid input, sorry");

    // Input value b
    let mut input2 = String::new();
    println!("Please Enter value b:");
    io::stdin().read_line(&mut input2).expect("Failed to read input, sorry");
    let b: f64 = input2.trim().parse().expect("Invalid input, sorry");

    // Input value c
    let mut input3 = String::new();
    println!("Please Enter value c:");
    io::stdin().read_line(&mut input3).expect("Failed to read input, sorry");
    let c: f64 = input3.trim().parse().expect("Invalid input, sorry");

    // Display the equation
    println!("\nYour Equation: {}x² + {}x + {} = 0", a, b, c);

    // Calculate discriminant
    let discriminant = b * b - 4.0 * a * c;
    println!("Discriminant = {}", discriminant);

    // Find out and display roots based on discriminant
    if discriminant > 0.0 {
        println!("\nThe discriminant is positive, so there are two distinct real roots:");
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if discriminant == 0.0 {
        println!("\nThe discriminant is zero, so there is only one real root:");
        let root = -b / (2.0 * a);
        println!("Root = {}", root);
    } else {
        println!("\nThe discriminant is negative, so there are no real roots.");
        println!("The roots are complex numbers:");
        let real_part = -b / (2.0 * a);
        let imaginary_part = (-discriminant).sqrt() / (2.0 * a);
        println!("Root 1 = {} + {}i", real_part, imaginary_part);
        println!("Root 2 = {} - {}i", real_part, imaginary_part);
    }
}