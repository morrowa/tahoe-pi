// rust interface to gps.h
// c library for communicating with GPSD

use std::libc::c_int;
use std::libc::c_double;
use std::libc::c_void;
use std::libc::uint64_t;
use std::libc::c_char;
use std::libc::c_uint;

static MAXCHANNELS: u32 = 72;
static MAXTAGLEN: u32 = 8;
static GPS_PATH_MAX: u32 = 128;

pub struct gps_fix_t {
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

pub struct dop_t {
	xdop: c_double,
	ydop: c_double,
	pdop: c_double,
	hdop: c_double,
	vdop: c_double,
	tdop: c_double,
	gdop: c_double
}

pub struct devconfig_t {
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

pub struct policy_t {
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

pub struct gps_data_t {
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
	pub fn gps_open(server: *c_char, port: *c_char, gps_data: &gps_data_t) -> c_int;
	pub fn gps_read(gps_data: &gps_data_t) -> c_int;
	pub fn gps_close(gps_data: &gps_data_t) -> c_int;

	pub fn earth_distance(lat1: c_double, lon1: c_double, lat2: c_double, lon2: c_double) -> c_double;
}

