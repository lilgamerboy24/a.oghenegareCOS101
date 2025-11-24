fn main() {
    // Name vector
    let name = vec!["Mary", "Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];

    // Age vector
    let age = vec![16, 17, 19, 22, 20, 21, 18, 23];

    println!("\nAge allocation:\n");

    // Loop to iterate elements in vector
    for i in 0..age.len() {
        // Iterating through i on the vector
        println!("{} is {} years old", name[i], age[i]);
    }
}