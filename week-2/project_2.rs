fn main() {
	let t:f64 = 450000.0;
	let m:f64 = 1500000.0;
	let h:f64 = 750000.0;
	let d:f64 = 2850000.0;
	let a:f64 = 250000.0;

	//average
	let av = ( t + m + h + d + a ) / 5.0;
	println!("Average is {}", av);
	let sm = t + m + h + d + a;
	println!("sum is {}", sm)
}