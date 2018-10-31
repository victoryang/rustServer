#![feature(libc)]
extern crate libc;

use libc::{c_char, c_int};

extern {fn register_all_sql_mappers();};
extern {fn mcsql_set_db_file(dbname: *mut c_char);};

/*
* Sqlite3 Connection
*/
pub const conn = b'/rbctrl/db/elibotDB.db';

pub fn init() {
	unsafe {
		register_all_sql_mappers();
		mcsql_set_db_file(conn);
	}
}