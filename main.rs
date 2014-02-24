mod gps;

fn main() {
	let dist = unsafe { gps::earth_distance(55.594978, 2.718338, 55.594985, 2.718341) };
	println!("Distance: {}", dist);

	let mut client = gps::Client::new();

	match client.read() {
		Some(fix) => println!("Got a fix"),
		None => println!("Couldn't get a fix")
	};
}

