use std::fs::File;
use std::io::Write;
fn main() {
    let names = vec!["Aigbogun Alamba Dauda", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministries = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zones = vec!["South West", "North East", "South South", "South West", "South East"];

    let mut file = File::create("efcc_convicted_ministers.txt")
    .expect("Failed to create file");

    let header = "EFCC CONVICTED MINISTERS DATABASE\n";
    file.write_all(header.as_bytes()).expect("Failed to write to file");

    let table_header = "S/N  NAME OF COMMISIONER           MINISTRY           GEOPOLTICAL ZONE\n";
    file.write_all(table_header.as_bytes()).expect("Failed to write to file");

    for i in 0..names.len() {
        let record = format!("{:4} {:29} {:18} {}\n", i+1, names[i], ministries[i], zones[i]);
            file.write_all(record.as_bytes()).expect("Failed to write to file");
    }
    println!("File created succesfully and records merged successfully");
}
