use std::io;
use std::f64::consts::PI;

// Function to calculate area of trapezium
fn area_of_trapezium() -> f64 {
    println!("\n📐 Calculating Area of Trapezium");
    println!("Formula: height/2 * (base1 + base2)");
    
    let height = get_positive_input("Enter height: ");
    let base1 = get_positive_input("Enter base1: ");
    let base2 = get_positive_input("Enter base2: ");
    
    (height / 2.0) * (base1 + base2)
}

// Function to calculate area of rhombus
fn area_of_rhombus() -> f64 {
    println!("\n🔷 Calculating Area of Rhombus");
    println!("Formula: ½ × diagonal1 × diagonal2");
    
    let diagonal1 = get_positive_input("Enter diagonal1: ");
    let diagonal2 = get_positive_input("Enter diagonal2: ");
    
    0.5 * diagonal1 * diagonal2
}

// Function to calculate area of parallelogram
fn area_of_parallelogram() -> f64 {
    println!("\n📏 Calculating Area of Parallelogram");
    println!("Formula: base × altitude");
    
    let base = get_positive_input("Enter base: ");
    let altitude = get_positive_input("Enter altitude: ");
    
    base * altitude
}

// Function to calculate area of cube
fn area_of_cube() -> f64 {
    println!("\n🧊 Calculating Surface Area of Cube");
    println!("Formula: 6 × (length of side)²");
    
    let side = get_positive_input("Enter length of side: ");
    
    6.0 * side.powi(2)
}

// Function to calculate volume of cylinder
fn volume_of_cylinder() -> f64 {
    println!("\n🛢️ Calculating Volume of Cylinder");
    println!("Formula: π × radius² × height");
    
    let radius = get_positive_input("Enter radius: ");
    let height = get_positive_input("Enter height: ");
    
    PI * radius.powi(2) * height
}

// Helper function to get positive numeric input from user
fn get_positive_input(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::Write::flush(&mut io::stdout()).expect("Flush failed");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim().parse::<f64>() {
            Ok(value) if value > 0.0 => {
                return value;
            }
            Ok(_) => {
                println!("❌ Please enter a positive number greater than 0!");
            }
            Err(_) => {
                println!("❌ Please enter a valid number!");
            }
        }
    }
}

// Function to display the main menu
fn display_menu() {
    println!("\n{}", "=".repeat(60));
    println!("{:^60}", "MTH 101 - GEOMETRY CALCULATOR");
    println!("{}", "=".repeat(60));
    println!("Please select a shape to calculate:");
    println!("1. 📐 Area of Trapezium");
    println!("2. 🔷 Area of Rhombus");
    println!("3. 📏 Area of Parallelogram");
    println!("4. 🧊 Surface Area of Cube");
    println!("5. 🛢️ Volume of Cylinder");
    println!("6. ❌ Exit Program");
    print!("Enter your choice (1-6): ");
}

// Function to get user's menu choice
fn get_menu_choice() -> u32 {
    loop {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        
        match choice.trim().parse::<u32>() {
            Ok(value) if (1..=6).contains(&value) => {
                return value;
            }
            _ => {
                println!("❌ Please enter a valid number between 1 and 6!");
                print!("Enter your choice (1-6): ");
                io::Write::flush(&mut io::stdout()).expect("Flush failed");
            }
        }
    }
}

fn main() {
    println!("{}", "🎓".repeat(30));
    println!("{:^60}", "WELCOME TO MTH 101 GEOMETRY CALCULATOR");
    println!("{}", "🎓".repeat(30));
    println!("Developed for Professor: MTH 101 Mathematics Department");
    
    loop {
        display_menu();
        io::Write::flush(&mut io::stdout()).expect("Flush failed");
        
        let choice = get_menu_choice();
        
        if choice == 6 {
            println!("\n{}", "👋".repeat(20));
            println!("Thank you for using MTH 101 Geometry Calculator!");
            println!("Goodbye! 👋");
            break;
        }
        
        let result = match choice {
            1 => area_of_trapezium(),
            2 => area_of_rhombus(),
            3 => area_of_parallelogram(),
            4 => area_of_cube(),
            5 => volume_of_cylinder(),
            _ => continue, // This should never happen due to validation
        };
        
        // Display the result
        println!("\n{}", "✨".repeat(40));
        println!("📊 CALCULATION RESULT");
        println!("{}", "✨".repeat(40));
        println!("Result: {:.4}", result);
        println!("Rounded: {:.2}", result);
        
        // Wait for user to continue
        println!("\nPress Enter to continue...");
        let mut _continue = String::new();
        io::stdin().read_line(&mut _continue).expect("Failed to read input");
    }
}

// Unit tests for the geometric calculations
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_area_of_trapezium() {
        // Test with known values: height=5, base1=3, base2=7
        // Expected: (5/2) * (3+7) = 2.5 * 10 = 25
        let result = (5.0 / 2.0) * (3.0 + 7.0);
        assert_eq!(result, 25.0);
    }

    #[test]
    fn test_area_of_rhombus() {
        // Test with known values: diagonal1=8, diagonal2=6
        // Expected: 0.5 * 8 * 6 = 24
        let result = 0.5 * 8.0 * 6.0;
        assert_eq!(result, 24.0);
    }

    #[test]
    fn test_area_of_parallelogram() {
        // Test with known values: base=10, altitude=4
        // Expected: 10 * 4 = 40
        let result = 10.0 * 4.0;
        assert_eq!(result, 40.0);
    }

    #[test]
    fn test_area_of_cube() {
        // Test with known values: side=3
        // Expected: 6 * 3² = 6 * 9 = 54
        let result = 6.0 * 3.0_f64.powi(2);
        assert_eq!(result, 54.0);
    }

    #[test]
    fn test_volume_of_cylinder() {
        // Test with known values: radius=2, height=5
        // Expected: π * 2² * 5 = π * 4 * 5 = 20π ≈ 62.8319
        let result = PI * 2.0_f64.powi(2) * 5.0;
        let expected = 20.0 * PI;
        assert!((result - expected).abs() < 0.0001);
    }
}

// Additional demonstration function
fn demonstrate_calculations() {
    println!("\n{}", "📚".repeat(40));
    println!("{:^40}", "SAMPLE CALCULATIONS DEMONSTRATION");
    println!("{}", "📚".repeat(40));
    
    // Demonstrate each calculation with sample values
    let trapezium_sample = (5.0 / 2.0) * (3.0 + 7.0);
    println!("Trapezium (h=5, b1=3, b2=7): Area = {:.2}", trapezium_sample);
    
    let rhombus_sample = 0.5 * 8.0 * 6.0;
    println!("Rhombus (d1=8, d2=6): Area = {:.2}", rhombus_sample);
    
    let parallelogram_sample = 10.0 * 4.0;
    println!("Parallelogram (b=10, a=4): Area = {:.2}", parallelogram_sample);
    
    let cube_sample = 6.0 * 3.0_f64.powi(2);
    println!("Cube (side=3): Surface Area = {:.2}", cube_sample);
    
    let cylinder_sample = PI * 2.0_f64.powi(2) * 5.0;
    println!("Cylinder (r=2, h=5): Volume = {:.2}", cylinder_sample);
}