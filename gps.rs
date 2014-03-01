// gps.rs
// Uses a Unix socket to connect to gpsd.

use extra::arc::RWArc;

pub struct Fix {
	time: f64,
	lat: f64,
	lon: f64,
	alt: f64
}

pub struct Client {
	priv fix: RWArc<Option<~Fix>>,
}

impl Client {
	pub fn new() -> Client {
		let client = Client { fix: RWArc.new(None) };
		let background_storage = client.fix.clone();
		do spawn {
			gpsd_listener(background_storage);
		};
		client
	}
	// TODO add closure-based reading of fix
	// TODO implement destructor including halting background task
}

fn gpsd_listener(storage: RWArc<Option<Fix>>) {
	// TODO open Unix socket to gpsd and read until termination message from foreground
}

