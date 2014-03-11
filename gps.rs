// rust interface to gps_rust.h
// simplified c library for communicating with GPSD

use std::ptr;
use std::libc::{c_int, c_double, c_void};


struct gps_rust_fix {
	mode: c_int,
	time: c_double,
	latitude: c_double,
	longitude: c_double,
	altitude: c_double
}


pub struct Fix {
	time: f64,
	mode: u8,
	latitude: f64,
	longitude: f64,
	altitude: f64
}


pub struct Client {
	priv state: *c_void,
	priv fix: gps_rust_fix
}


#[link(name="gps_rust")]
extern {
	fn gps_rust_open() -> *c_void;
	fn gps_rust_read(state: *c_void, out_fix: &mut gps_rust_fix) -> c_int;
	fn gps_rust_close(state: *c_void);
}


impl Client {
	pub fn new() -> Client {
		Client {
			state: unsafe { let st = gps_rust_open(); if st == ptr::null() { fail!("could not connect"); } st },
			fix: gps_rust_fix { time: 0.0, mode: 0, latitude: 0.0, longitude: 0.0, altitude: 0.0 }
		}
	}

	pub fn read(&mut self) -> Option<Fix> {
		let bytes_read = unsafe { gps_rust_read(self.state, &mut (self.fix)) };

		if bytes_read >= 0 {
			Some(Fix {
				time: self.fix.time as f64,
				mode: self.fix.mode as u8,
				latitude: self.fix.latitude as f64,
				longitude: self.fix.longitude as f64,
				altitude: self.fix.altitude as f64
			})
		} else {
			None
		}
	}
}

#[unsafe_destructor]
impl Drop for Client {
	fn drop(&mut self) {
		unsafe {
			gps_rust_close(self.state);
		}
	}
}

