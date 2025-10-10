fn main() {
	let n: f64 = 5.0; // the year
	let r: f64 = 10.0; // the rate
	let p: f64 = 520000000.0; // the principal
	let x: f64 = 1.0 + r / 100.0; // i broke things down in the formula and calculated what was only in the square bracket
	let a: f64 = p * x.powf(n); // the formula for amount
	let c: f64 = a - p; // how to solve compound interest , the formula for it
	println!("compound interest = {}" , c); 
	println!("amount = {}" , a);
}