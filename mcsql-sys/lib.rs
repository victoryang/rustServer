#![feature(libc)]
extern crate libc;

use libc::{c_char, c_int};
use std::ffi::CString;

/*
* For initialization
*/
extern {fn register_all_sql_mappers();}
extern {fn mcsql_set_db_file(dbname: *const c_char);}

/*
* Database handle
*/
extern {fn mcsql_arc_get_all() -> *mut c_char;}

pub fn arc_get_all() -> String {
	let c_string = unsafe { CString::from_raw(mcsql_arc_get_all()) };
	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {},
	}
}

pub fn init() {
	let conn = CString::new("/rbctrl/db/elibotDB.db").unwrap();
	unsafe {
		register_all_sql_mappers();
		mcsql_set_db_file(conn.as_ptr());
	}
}