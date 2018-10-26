#![feature(libc)]
extern crate libc;
extern crate crc;

use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use crc::crc32;

extern {fn init_nv_ram();}
extern {fn mrj_init();}
extern {fn get_resource_data() -> *mut c_char;}

static mut CRC_SHARED: u32 = 0;

pub fn get_shared() -> Option<CString> {
	let c_string = unsafe { CString::from_raw(get_resource_data()) };
	let crc = crc32::checksum_ieee(c_string.to_bytes());

	if crc != CRC_SHARED {
		CRC_SHARED = crc;
		return Option::Some(c_string);
	}
	
	Option::None
}

pub fn init_worker_resource() {
	unsafe {
		init_nv_ram();
		mrj_init();
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
