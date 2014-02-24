// rust interface to gps.h
// c library for communicating with GPSD

use std::c_str;
use std::ptr;
use std::mem;
use std::libc::{c_int, c_double, c_void, uint64_t, c_char, c_uint, malloc, free, size_t};

static MAXCHANNELS: u32 = 72;
static MAXTAGLEN: u32 = 8;
static GPS_PATH_MAX: u32 = 128;

struct gps_fix_t {
	time: c_double,
	mode: c_int,
	ept: c_double,
	latitude: c_double,
	epy: c_double,
	longitude: c_double,
	epx: c_double,
	altitude: c_double,
	epv: c_double,
	track: c_double,
	epd: c_double,
	speed: c_double,
	eps: c_double,
	climb: c_double,
	epc: c_double
}

struct dop_t {
	xdop: c_double,
	ydop: c_double,
	pdop: c_double,
	hdop: c_double,
	vdop: c_double,
	tdop: c_double,
	gdop: c_double
}

struct devconfig_t {
	path: [c_char, ..GPS_PATH_MAX],
	flags: c_int,
	driver: [c_char, ..64],
	subtype: [c_char, ..64],
	activated: c_double,
	baudrate: c_uint,
	stopbits: c_uint,
	parity: c_char,
	cycle: c_double,
	mincycle: c_double,
	driver_mode: c_int
}

struct policy_t {
	watcher: c_int,
	json: c_int,
	nmea: c_int,
	raw: c_int,
	scaled: c_int,
	timing: c_int,
	loglevel: c_int,
	devpath: [c_char, ..GPS_PATH_MAX],
	remote: [c_char, ..GPS_PATH_MAX]
}

struct gps_data_t {
	set: uint64_t,
	online: c_double,
	gps_fd: *c_void,
	fix: gps_fix_t,
	separation: c_double,
	status: c_int,
	satellites_used: c_int,
	used: [c_int, ..MAXCHANNELS],
	dop: dop_t,
	epe: c_double,
	skyview_time: c_double,
	satellites_visible: c_int,
	PRN: [c_int, ..MAXCHANNELS],
	elevation: [c_int, ..MAXCHANNELS],
	azimuth: [c_int, ..MAXCHANNELS],
	ss: [c_double, ..MAXCHANNELS],
	dev: devconfig_t,
	policy: policy_t,
	tag: [c_char, ..MAXTAGLEN],
	error: [c_char, ..5664], // there's a union in the struct that we don't care about
	privdata: *c_void
}

#[link(name = "gps")]
extern {
	fn gps_open(server: *c_char, port: *c_char, gps_data: *mut gps_data_t) -> c_int;
	fn gps_read(gps_data: *mut gps_data_t) -> c_int;
	fn gps_close(gps_data: *mut gps_data_t) -> c_int;

	pub fn earth_distance(lat1: c_double, lon1: c_double, lat2: c_double, lon2: c_double) -> c_double;
}

pub struct Fix {
	time: f64,
	mode: u8,
	latitude: f64,
	longitude: f64,
	altitude: f64
}

pub struct Client {
	priv state: *gps_data_t
}

impl Client {
	pub fn new() -> Client {
		let client = unsafe { Client { state: malloc(mem::size_of::<gps_data_t>() as size_t) as *gps_data_t } };
		client.open();
		client
	}

	fn open(&self) {
		let hostname = "shared memory";
		let c_hostname = hostname.to_c_str();

		let return_code = c_hostname.with_ref(|c_buffer| {
			unsafe { gps_open(c_buffer, ptr::null(), self.state as *mut gps_data_t) }
		});

		if return_code != 0 {
			fail!("gps::Client::new() could not connect to gpsd");
		}
	}

	pub fn read(&self) -> Option<Fix> {
		let bytes_read = unsafe { gps_read(self.state as *mut gps_data_t) };

		if bytes_read >= 0 {
			Some(Fix {time: 1.0, mode: 1, latitude: 2.0, longitude: 2.0, altitude: 2.0})
		} else {
			None
		}
	}
}

#[unsafe_destructor]
impl Drop for Client {
	fn drop(&mut self) {
		unsafe {
			gps_close(self.state as *mut gps_data_t);
			free(self.state as *c_void)
		}
	}
}

