fn main() {
    let fullname = " Pan-Atlantic University ";

    println!("Name: {}", fullname);
    println!("Length is {}", fullname.len());

    // trim leading/trailing spaces
    println!("Before trim: {}", fullname);

    let trimmed = fullname.trim();
    println!("After trim: {}", trimmed);
    println!("Length is {}", trimmed.len());
}