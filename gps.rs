// gps.rs
// Uses a network socket to connect to gpsd.

extern crate sync;
extern crate serialize;
extern crate time;

use self::sync::RWArc;
use self::serialize::json::{Parser, Json, Object, Number};
use self::time::{strptime, Timespec};
use std::io::{Stream, TcpStream, BufferedStream, IoResult};
use std::io::net::ip::SocketAddr;
use std::comm::{Disconnected, Data};

#[deriving(Clone)]
pub enum GpsMode {
	NoModeSeen = 0,
	NoFix = 1,
	Fix2d = 2,
	Fix3d = 3
}

#[deriving(Clone)]
pub struct Fix {
	mode: GpsMode,
	time: Option<Timespec>,
	lat:  Option<f64>,
	lon:  Option<f64>,
	alt:  Option<f64>
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

	if major_vers != 3 {
		fail!("error: unsupported gpsd protocol version {}", major_vers);
	}

	gpsd_subscribe(&mut stream).unwrap();

	loop {
		match halt_port.try_recv() {
			Data(val) if val => break,
			Disconnected => break,
			_ => {}
		}

		let line = stream.read_line().unwrap();

		match Parser::new(line.as_slice().chars()).parse().ok().and_then(|j| parse_gpsd_response(&j)) {
			Some(fix) => storage.write(|opt_ref| *opt_ref = Some(~fix)),
			_ => {}
		};
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

fn gpsd_subscribe<S: Stream>(stream: &mut BufferedStream<S>) -> IoResult<()> {
	stream.write_line(&"?WATCH={\"enable\":true,\"json\":true}").and(stream.flush())
}

fn parse_gpsd_response(response: &Json) -> Option<Fix> {
	match response.find(&~"class").and_then(|c| c.as_string()) {
		Some(class) if class == "TPV" => {},
		_ => return None
	};
	Some(Fix {
		mode: match response.find(&~"mode").and_then(|m| m.as_number()) {
			      Some(m) => match m {
				      1f64 => NoFix,
				      2f64 => Fix2d,
				      3f64 => Fix3d,
				      _ => NoModeSeen
			      },
			      _ => NoModeSeen
		      },
		time: response.find(&~"time").and_then(|t| t.as_string()).and_then(|t| strptime(t, "%FT%T.%fZ").ok()).and_then(|t| Some(t.to_timespec())),
		lat: response.find(&~"lat").and_then(|l| l.as_number()),
		lon: response.find(&~"lon").and_then(|l| l.as_number()),
		alt: response.find(&~"alt").and_then(|a| a.as_number())
	})
}

