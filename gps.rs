// gps.rs
// Uses a Unix socket to connect to gpsd.

extern mod extra;
use self::extra::arc::RWArc;

#[deriving(Clone)]
pub struct Fix {
	time: f64,
	lat: f64,
	lon: f64,
	alt: f64
}

pub struct Client {
	priv fix: RWArc<Option<~Fix>>,
	priv bg_chan: Chan<bool>
}

impl Client {
	pub fn new() -> Client {
		let (port, chan) = Chan::new();
		let client = Client { fix: RWArc::new(None), bg_chan: chan };
		let background_storage = client.fix.clone();
		spawn(proc() { gpsd_listener(background_storage, port); });
		client
	}

	pub fn read(&self) -> Option<~Fix> {
		self.fix.read(|data: &Option<~Fix>| -> Option<~Fix> {
			data.clone()
		})
	}
}

impl Drop for Client {
	fn drop(&mut self) {
		self.bg_chan.try_send(true);
	}
}

fn gpsd_listener(storage: RWArc<Option<~Fix>>, halt_port: Port<bool>) {
	// TODO open Unix socket to gpsd and read until termination message from foreground
	loop {
		//...program code here

		match halt_port.try_recv() {
			Some(stop) if stop => break,
			_ => continue
		};
	}
}

