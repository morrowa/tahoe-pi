// gps.rs
// Uses a network socket to connect to gpsd.

extern crate sync;
extern crate serialize;

use self::sync::RWArc;
use self::serialize::json::{Parser, Json, Object, Number};
use std::io::{Stream, TcpStream, BufferedStream};
use std::io::net::ip::SocketAddr;
use std::comm::{Disconnected, Data};

#[deriving(Clone)]
pub struct Fix {
	time: f64,
	lat: f64,
	lon: f64,
	alt: f64
}

pub struct Client {
	priv fix: RWArc<Option<~Fix>>,
	priv bg_chan: Sender<bool>
}

impl Client {
	pub fn connect(addr: &SocketAddr) -> Client {
		let (tx, rx) = channel();
		let client = Client { fix: RWArc::new(None), bg_chan: tx };
		let background_storage = client.fix.clone();
		let addr_copy = ~addr.clone();
		spawn(proc() { gpsd_listener(addr_copy, background_storage, rx); });
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

fn gpsd_listener(addr: ~SocketAddr, storage: RWArc<Option<~Fix>>, halt_port: Receiver<bool>) {
	let mut stream = gpsd_connect(addr);

	let (major_vers, _) = gpsd_init(&mut stream);

	println!("Got version {}", major_vers);

	if major_vers != 3 {
		fail!("error: unsupported gpsd protocol version {}", major_vers);
	}

	gpsd_subscribe(&mut stream);

	loop {
		match halt_port.try_recv() {
			Data(val) if val => break,
			Disconnected => break,
			_ => {}
		}

		println!("trying to read from gpsd...");

		let line = stream.read_line().unwrap();

		print!("Got message from gpsd: {}", line);

		parse_gpsd_response(line, &storage);
	}
}

fn gpsd_connect(addr: ~SocketAddr) -> BufferedStream<TcpStream> {
	BufferedStream::with_capacities(1536, 81, TcpStream::connect(*addr).unwrap())
}

fn gpsd_init<S: Stream>(stream: &mut BufferedStream<S>) -> (u32,u32) {
	let raw_protocol_json = stream.read_line().unwrap();
	let protocol_json: Json = Parser::new(raw_protocol_json.as_slice().chars()).parse().unwrap();

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

fn gpsd_subscribe<S: Stream>(stream: &mut BufferedStream<S>) {
	stream.write_line(&"?WATCH={\"enable\":true,\"json\":true}");
	stream.flush();
}

fn parse_gpsd_response(response: &str, fix_storage: &RWArc<Option<~Fix>>) {
}

