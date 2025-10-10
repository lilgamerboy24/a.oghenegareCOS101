fn main() {
	let p: f64 = 510_000.00;
	let r: f64 = 5.00;
	let n: f64 = 3.00;
	let x: f64 = 1.00 - r / 100.00;
	let c: f64 = p * x.powf(n);

	println!("The value = {}" , c);
}