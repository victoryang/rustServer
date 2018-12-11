#![feature(libc)]
extern crate libc;
extern crate crc;

use libc::{c_char, c_int};
use std::ffi::CString;
use crc::crc32;

extern {fn init_nv_ram();}
extern {fn mrj_init();}

extern {fn mrj_get_plc() -> *mut c_char;}
extern {fn mrj_get_resource() -> *mut c_char;}
extern {fn mrj_get_nv() -> *mut c_char;}
extern {fn mrj_get_sysvar(datatype: c_int, start: c_int, end: c_int) -> *mut c_char;}
extern {fn mrj_get_locvar(datatype: c_int, number: c_int, start: c_int, end: c_int) -> *mut c_char;}

// api for specific variables
extern {fn mrj_get_remote_mode_status() -> c_int;}

static mut CRC_PLC: u32 = 0;
static mut CRC_SHARED: u32 = 0;
static mut CRC_NV: u32 = 0;

pub fn get_plc() -> Option<CString> {
	let c_string = unsafe { CString::from_raw(mrj_get_plc()) };
	let crc = crc32::checksum_ieee(c_string.to_bytes());

	unsafe {
		if crc != CRC_PLC {
			CRC_PLC = crc;
			return Option::Some(c_string);
		}
	}

	Option::None
}

pub fn get_shared() -> Option<CString> {
	let c_string = unsafe { CString::from_raw(mrj_get_resource()) };
	let crc = crc32::checksum_ieee(c_string.to_bytes());

	unsafe {
		if crc != CRC_SHARED {
			CRC_SHARED = crc;
			return Option::Some(c_string);
		}
	}

	Option::None
}

pub fn get_nv() -> Option<CString> {
	let c_string = unsafe { CString::from_raw(mrj_get_nv()) };
	let crc = crc32::checksum_ieee(c_string.to_bytes());

	unsafe {
		if crc != CRC_NV {
			CRC_NV = crc;
			return Option::Some(c_string);
		}
	}

	Option::None
}

// Turn C result into String, responded to caller
fn result_into_string_response(c_result: *mut c_char) -> String {
	if c_result.is_null() {
		return String::from("null");
	}

	let c_string = unsafe { CString::from_raw(c_result) };
	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("fails to get result");
		},
	}
}

pub fn get_plc_once() -> String {
	let c_result = unsafe { mrj_get_plc() };
	
	result_into_string_response(c_result)
}

pub fn get_shared_once() -> String {
	let c_result = unsafe { mrj_get_resource() };
	
	result_into_string_response(c_result)
}

pub fn get_nv_once() -> String {
	let c_result = unsafe { mrj_get_nv() };
	
	result_into_string_response(c_result)
}

pub fn get_system_variables(datatype: i32, start: i32, end: i32) -> String {
	let c_result = unsafe { mrj_get_sysvar(datatype, start, end) };
	
	result_into_string_response(c_result)
}

pub fn get_local_variables(datatype: i32, number: i32, start: i32, end: i32) -> String {
	let c_result = unsafe { mrj_get_locvar(datatype, number, start, end) };
	
	result_into_string_response(c_result)
}

pub fn get_remote_mode_status() -> i32 {
	unsafe { mrj_get_remote_mode_status() }
}

pub fn init_worker_resource() {
	unsafe {
		mrj_init();
		init_nv_ram();
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
