fn main() {
    let toshiba: f64 = 450000.0;
    let mac: f64 = 1500000.0;
    let hp: f64 = 750000.0;
    let dell: f64 = 2850000.0;
    let acer: f64 = 250000.0;

    // sum
    let sum = toshiba + mac + hp + dell + acer;
    println!("Total Sales Amount = N{}", sum);

    // average
    let average = sum / 5.0;
    println!("Average Sales Amount = N{}", average);
}