fn main() {
	let b: f64 = 1500000.00; // i will put all the amount in variables each
	let a: f64 = 450000.00;
	let c: f64 = 750000.00;
	let d: f64 = 2850000.00;
	let e: f64 = 250000.00;

	let sum: f64 = a + b + c + d + e; // i will sum all the variables to get sum

	let f: f64 = 5.0; //the number of goods
	let average: f64 = sum / f;

	println!("The sum of the sales record = {}" , sum);
	println!("the average of the sales record = {}" , average);
}