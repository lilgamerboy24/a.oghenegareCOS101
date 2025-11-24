fn main() {
    let k1 = "Yemisi".to_string();
    let k2 = "Shyllon".to_string();
    let k3 = "Museum".to_string();
    let k4 = "Lagos".to_string();
    let k5 = "Art".to_string();
    let k6 = "PAU".to_string();

    // format macro to create a single formatted string
    let k7 = format!("{} {} {} {} {} {}", k1, k2, k3, k4, k5, k6);

    // print the formatted output
    println!("\n{}", k7);
}