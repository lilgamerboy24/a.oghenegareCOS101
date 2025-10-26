use std::io;

fn main(){
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter a:");
    io::stdin().read_line(&mut a).expect("Failed to read line");

    println!("Enter b:");
    io::stdin().read_line(&mut b).expect("Faield to read line");

    println!("Enter c:");
    io::stdin().read_line(&mut c).expect("Failed to read line");

    let a:f64 = a.trim().parse().expect("Enter a valid number");
    let b:f64 = b.trim().parse().expect("Enter a valid number");
    let c:f64 = c.trim().parse().expect("Enter a valid number");

    let discriminant:f64 = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let x1:f64 = (-b + discriminant.sqrt()) / 2.0 * a;
        let x2:f64 = (-b - discriminant.sqrt()) / 2.0 * a;
        println!("x1 = {} x2 = {} ",x1,x2);
    } else if discriminant < 0.0 {
        println!("Error");
    }else {
        println!("zero");
    }
}
