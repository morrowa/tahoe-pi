// gps.rs
// Uses a Unix socket to connect to gpsd.

extern mod extra;
use self::extra::arc::RWArc;
use self::extra::json::{Parser, Json, Object, Number};

use std::io::Stream;
use std::io::net::tcp::TcpStream;
use std::io::net::ip::SocketAddr;
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
	pub fn connect(addr: &SocketAddr) -> Client {
		let (port, chan) = Chan::new();
		let client = Client { fix: RWArc::new(None), bg_chan: chan };
		let background_storage = client.fix.clone();
		let addr_copy = ~addr.clone();
		spawn(proc() { gpsd_listener(addr_copy, background_storage, port); });
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

fn gpsd_listener(addr: ~SocketAddr, storage: RWArc<Option<~Fix>>, halt_port: Port<bool>) {
	let mut stream = gpsd_connect(addr);

	let (major_vers, _) = gpsd_init(&mut stream);

	println!("Got version {}", major_vers);

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

fn gpsd_connect(addr: ~SocketAddr) -> BufferedStream<TcpStream> {
	BufferedStream::with_capacities(1536, 81, TcpStream::connect(*addr).unwrap())
}

fn gpsd_init<S: Stream>(stream: &mut BufferedStream<S>) -> (u32,u32) {
	let protocol_json: Json = match stream.read_line() {
		Some(string) => Parser::new(string.as_slice().chars()).parse().unwrap(),
		None => fail!("no protocol response from gpsd")
	};

	let root_obj = match protocol_json {
		Object(obj) => obj,
		_ => fail!("invalid protocol response")
	};

	let protocol_major = match *(root_obj.find(&~"proto_major").unwrap()) {
		Number(val) => val as u32,
		_ => fail!("invalid protocol response")
	};

	let protocol_minor = match *(root_obj.find(&~"proto_minor").unwrap()) {
		Number(val) => val as u32,
		_ => fail!("invalid protocol response")
	};

	(protocol_major, protocol_minor)
}

