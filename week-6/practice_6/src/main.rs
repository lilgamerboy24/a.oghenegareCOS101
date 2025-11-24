fn main() {
    // Create strings
    let n1 = "Electrical".to_string();
    let n2 = "Electronics".to_string();
    let n3 = "Engineering".to_string();

    // Concatenate using + and to_string (or + with &str)
    let mut w1 = "Electrical".to_string();
    w1.push_str(" & ");
    w1.push_str("Engineering");

    // Another concatenation example
    let w2 = "Science".to_string();

    // Print a descriptive message
    println!();
    println!("{} is aimed at developing competent, creative, innovative, entrepreneurial and ethically-minded persons, capable of creating value in the diverse fields of Computer Science.", w1);

    // Example of combining multiple strings for a longer message
    let full = format!("{} {} {}", n1, n2, n3);
    println!("{}", full);
}