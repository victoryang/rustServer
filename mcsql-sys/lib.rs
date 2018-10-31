#![feature(libc)]
extern crate libc;

use libc::{c_char, c_int};
use std::ffi::CString;

extern {fn register_all_sql_mappers();};
extern {fn mcsql_set_db_file(dbname: *const c_char);};

pub fn init() {
	let conn = CString::new(conn).unwrap();
	unsafe {
		register_all_sql_mappers();
		mcsql_set_db_file(conn.as_ptr());
	}
}