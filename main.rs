mod gps;

fn main() {
	let client = gps::Client::new();

	match client.read() {
		Some(real_fix) => println!("Got a fix at {}, {}", real_fix.lat, real_fix.lon),
		None => println!("Couldn't get a fix")
	};
}

