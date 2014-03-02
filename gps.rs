// gps.rs
// Uses a Unix socket to connect to gpsd.

extern mod extra;
use self::extra::arc::RWArc;

use std::io::Stream;
use std::io::net::unix::UnixStream;
use std::io::buffered::BufferedStream;

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
	pub fn connect(path: &Path) -> Client {
		let (port, chan) = Chan::new();
		let client = Client { fix: RWArc::new(None), bg_chan: chan };
		let background_storage = client.fix.clone();
		let path_copy = ~path.clone();
		spawn(proc() { gpsd_listener(path_copy, background_storage, port); });
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

fn gpsd_listener(path: ~Path, storage: RWArc<Option<~Fix>>, halt_port: Port<bool>) {
	let mut stream = gpsd_connect(path);

	let (major_vers, _) = gpsd_init(&mut stream);

	if major_vers != 3 {
		fail!("error: unsupported gpsd protocol version {}", major_vers);
	}

	loop {
		//...program code here

		match halt_port.try_recv() {
			Some(stop) if stop => break,
			_ => continue
		};
	}
}

fn gpsd_connect(path: ~Path) -> BufferedStream<UnixStream> {
	BufferedStream::with_capacities(1536, 81, UnixStream::connect(path).unwrap())
}

fn gpsd_init<S: Stream>(stream: &mut BufferedStream<S>) -> (u32,u32) {
	stream.write_str(&"?VERSION;\n");
	stream.flush();

	match stream.read_line() {
		Some(string) => println!("Got response: {}", string),
		None => fail!("didn't get anything")
	};
	(3, 1)
}

