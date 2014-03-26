use std::io::net::ip::SocketAddr;
use std::io;
use std::io::BufferedReader;

mod gps;

fn main() {
	let addr = SocketAddr {
		ip: from_str(&"127.0.0.1").unwrap(),
		port: 2947
	};
	let client = gps::Client::connect(&addr);

	println!("Press enter to read and quit.");

	let mut in_pipe = BufferedReader::new(io::stdin());

	let _ = in_pipe.read_line();

	match client.read() {
		Some(fix) => print_fix(fix),
		_ => println!("No fix")
	}
}

fn print_fix(fix: &gps::Fix) {
	let m: gps::GpsMode = fix.mode;
	match m {
		gps::Fix2d => {},
		gps::Fix3d => {},
		_ => { println!("No fix"); return }
	};

	println!("{}, {} ({})", fix.lat.unwrap(), fix.lon.unwrap(), fix.time.unwrap());
}

