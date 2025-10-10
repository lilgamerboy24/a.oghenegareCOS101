fn main() {
	let b: f64 = 1500000.00; // i will put all the amount in variables each
	let a: f64 = 450000.00;
	let c: f64 = 750000.00;
	let d: f64 = 2850000.00;
	let e: f64 = 250000.00;

	let sum: f64 = a + b + c + d + e; // i will sum all the variables to get sum

	let f: f64 = 2.0; // i put all the quantity in variables each to be summed
	let g: f64 = 1.0;
	let h: f64 = 3.0;
	let i: f64 = 3.0;
	let j: f64 = 1.0;
	let total_qty: f64 = f + g + h + i + j; // i summed all the quantity variables in order to get average
	let average: f64 = sum / total_qty; // i got the average

	println!("The sum of the sales record = {}" , sum);
	println!("the average of the sales record = {}" , average);
}