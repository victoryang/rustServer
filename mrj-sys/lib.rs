#![feature(libc)]
extern crate libc;
extern crate crc;

use libc::c_char;
use std::ffi::CString;
use crc::crc32;

extern {fn init_nv_ram();}
extern {fn mrj_init();}
extern {fn get_plc_data() -> *mut c_char;}
extern {fn get_resource_data() -> *mut c_char;}
extern {fn get_nv_data() -> *mut c_char;}

static mut CRC_PLC: u32 = 0;
static mut CRC_SHARED: u32 = 0;
static mut CRC_NV: u32 = 0;

pub fn get_plc() -> Option<CString> {
	let c_string = unsafe { CString::from_raw(get_plc_data()) };
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
	let c_string = unsafe { CString::from_raw(get_resource_data()) };
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
	let c_string = unsafe { CString::from_raw(get_nv_data()) };
	let crc = crc32::checksum_ieee(c_string.to_bytes());

	unsafe {
		if crc != CRC_NV {
			CRC_NV = crc;
			return Option::Some(c_string);
		}
	}

	Option::None
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
