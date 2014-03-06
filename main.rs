use std::io::net::ip::SocketAddr;
use std::io;
use std::io::buffered::BufferedReader;

mod gps;

fn main() {
	let addr = SocketAddr {
		ip: from_str(&"127.0.0.1").unwrap(),
		port: 2947
	};
	let client = gps::Client::connect(&addr);

	println!("Press enter to read and quit.");

	let mut in_pipe = BufferedReader::new(io::stdin());

	in_pipe.read_line();

	match client.read() {
		Some(real_fix) => println!("Got a fix at {}, {}", real_fix.lat, real_fix.lon),
		None => println!("Couldn't get a fix")
	};
}

