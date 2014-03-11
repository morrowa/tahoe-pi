mod gps;

fn main() {
	let mut client = gps::Client::new();

	match client.read() {
		Some(fix) if fix.mode > 0 => println!("Got a fix at {}, {}", fix.latitude, fix.longitude),
		_ => println!("Couldn't get a fix")
	};
}

