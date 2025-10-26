use std::io;

fn main() {
    let mut experience = String::new();
    let mut age = String::new();

    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase();

    println!("Enter the age of the employee:");
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age: i32 = age.trim().parse().expect("Please enter a valid number");

    let incentive: i32;

    if experience == "yes" {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000 * 12; // per month converted to annual
        } else {
            incentive = 1_480_000; // assume midrange if not under 28 or over 40
        }
    } else {
        incentive = 100_000;
    }

    println!("The annual incentive of the employee is ₦{}", incentive);
}