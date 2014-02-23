mod gps;

fn main() {
	let dist = unsafe { gps::earth_distance(55.594978, 2.718338, 55.594985, 2.718341) };
	println!("Distance: {}", dist);
}

