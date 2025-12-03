use std::fs::File;
use std::io::Write;

fn main() {
    // Student data in arrays
    let names = ["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric_numbers = ["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let departments = ["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let levels = ["300", "100", "200", "200", "100"];
    
    // Create file and write
    let mut file = File::create("pau_student_records.txt")
        .expect("Failed to create file");
    
    // Write header
    let header = "PAU STUDENT MANAGEMENT SYSTEM\n";
    file.write_all(header.as_bytes()).expect("Failed to write to file");
    
    let table_header = "Name                 Matric Number   Department                   Level\n";
    file.write_all(table_header.as_bytes()).expect("Failed to write to file");
    
    // Write each student using loop
    for i in 0..names.len() {
        let student_data = format!("{:20} {:15} {:28} {}\n", names[i], matric_numbers[i], departments[i], levels[i]);
        file.write_all(student_data.as_bytes()).expect("Failed to write to file");
    }
    
    println!("\nFile created successfully: pau_student_records.txt");
}