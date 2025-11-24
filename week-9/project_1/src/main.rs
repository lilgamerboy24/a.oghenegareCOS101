use std::fs::File;
use std::io::Write;

fn main() {
    // Create arrays for each drink category
    let lagers = ["33 Export", "Desperados", "Goldberg", "Guider", "Heineken", "Star"];
    let stouts = ["Legend", "Turbo King", "Williams"];
    let non_alcoholics = ["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    // Create and open file for writing
    let mut file = File::create("nigerian_breweries_drinks.txt")
        .expect("Failed to create file");

    // Write header
    writeln!(file, "NIGERIAN BREWERIES PLC - DRINK PORTFOLIO")
        .expect("Failed to write to file");
    writeln!(file, "=========================================")
        .expect("Failed to write to file");
    writeln!(file, "Rich portfolio of high-quality drinks:")
        .expect("Failed to write to file");
    writeln!(file, "").expect("Failed to write to file");

    // Write Lager category
    writeln!(file, "LAGERS:")
        .expect("Failed to write to file");
    writeln!(file, "--------")
        .expect("Failed to write to file");
    for lager in lagers.iter() {
        writeln!(file, "• {}", lager)
            .expect("Failed to write to file");
    }
    writeln!(file, "").expect("Failed to write to file");

    // Write Stout category
    writeln!(file, "STOUTS:")
        .expect("Failed to write to file");
    writeln!(file, "--------")
        .expect("Failed to write to file");
    for stout in stouts.iter() {
        writeln!(file, "• {}", stout)
            .expect("Failed to write to file");
    }
    writeln!(file, "").expect("Failed to write to file");

    // Write Non-Alcoholic category
    writeln!(file, "NON-ALCOHOLIC DRINKS:")
        .expect("Failed to write to file");
    writeln!(file, "---------------------")
        .expect("Failed to write to file");
    for non_alcoholic in non_alcoholics.iter() {
        writeln!(file, "• {}", non_alcoholic)
            .expect("Failed to write to file");
    }

    // Write footer
    writeln!(file, "").expect("Failed to write to file");
    writeln!(file, "=========================================")
        .expect("Failed to write to file");
    writeln!(file, "Nigeria's Number One Choice Since 1946")
        .expect("Failed to write to file");

    // Print confirmation message
    println!("✅ Drink portfolio successfully saved to 'nigerian_breweries_drinks.txt'");
    println!("📊 Total Lagers: {}", lagers.len());
    println!("📊 Total Stouts: {}", stouts.len());
    println!("📊 Total Non-Alcoholic Drinks: {}", non_alcoholics.len());
    println!("📝 Total Categories: 3");
}