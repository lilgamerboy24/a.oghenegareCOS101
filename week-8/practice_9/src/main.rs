fn main() {
    let b: (i32, bool, f64) = (118, true, 18.9);
    print(b);
}

// pass the tuple as a parameter
fn print(x: (i32, bool, f64)) {
    println!("Inside print method");
    println!("{:?}", x);
}