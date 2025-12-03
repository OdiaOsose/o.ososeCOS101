use std::fs::File;
use std::io::Write;

fn main() {
    // Create the file
    let mut file = File::create("nigerian_breweries_drinks.txt")
        .expect("Failed to create file");
    
    // Write the content
    let data = "NIGERIAN BREWERIES PLC - DRINK CATEGORIES

LAGER           STOUT          NON-ALCOHOLIC
33 Export       Legend         Maltina
Desperados      Turbo King     Amstel Malta
Goldberg        Williams       Malta Gold
Gulder                         Fayrouz
Heineken
Star
";
    
    file.write_all(data.as_bytes()).expect("Failed to write to file");
    
    println!("File created successfully!");
}