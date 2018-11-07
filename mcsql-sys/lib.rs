#![feature(libc)]
extern crate libc;

use libc::{c_char, int32_t};
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
extern {fn mcsql_arc_get_params(file_no: int32_t, group: *const c_char) -> *mut c_char;}
extern {fn mcsql_bookprogram_get_all() -> *mut c_char;}
extern {fn mcsql_enum_get_all() -> *mut c_char;}
extern {fn mcsql_extaxis_get_all() -> *mut c_char;}
extern {fn mcsql_interference_get_all() -> *mut c_char;}
extern {fn mcsql_ios_get_all(group: *const c_char, lang: *const c_char, auth: int32_t, tech: int32_t) -> *mut c_char;}
extern {fn mcsql_metadata_get_all(lang: *const c_char) -> *mut c_char;}
extern {fn mcsql_params_get_params() -> *mut c_char;}
extern {fn mcsql_params_get_valid_param_by_id(md_id: *const c_char) -> *mut c_char;}
extern {fn mcsql_params_get_valid_param_by_group(group: *const c_char) -> *mut c_char;}
extern {fn mcsql_ref_get_all() -> *mut c_char;}
extern {fn mcsql_toolframe_get_all() -> *mut c_char;}
extern {fn mcsql_toolframe_get_by_toolno(tool_no: int32_t) -> *mut c_char;}
extern {fn mcsql_userframe_get_all() -> *mut c_char;}
extern {fn mcsql_userframe_get_by_userno(user_no: int32_t) -> *mut c_char;}
extern {fn mcsql_zeropoint_get_all() -> *mut c_char;}

pub fn arc_get_all() -> String {
	let c_string = unsafe { CString::from_raw(mcsql_arc_get_all()) };
	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn arc_get_params(file_no: i32, group: String) -> String {
	let group = match CString::new(group) {
		Ok(group) => group,
		Err(_) => {
			return String::from(""); 
		}
	};

	let c_string = unsafe { CString::from_raw(mcsql_arc_get_params(file_no, group.as_ptr())) };
	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn bookprogram_get_all() -> String {
	let c_string = unsafe { CString::from_raw(mcsql_bookprogram_get_all()) };
	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn enum_get_all() -> String {
	let c_string = unsafe { CString::from_raw(mcsql_enum_get_all()) };
	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn extaxis_get_all() -> String {
	let c_string = unsafe { CString::from_raw(mcsql_extaxis_get_all()) };
	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn interference_get_all() -> String {
	let c_string = unsafe { CString::from_raw(mcsql_interference_get_all()) };
	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn ios_get_all(group: String, lang: String, auth: i32, tech: i32) -> String {
	let group = match CString::new(group) {
		Ok(group) => group,
		Err(_) => {
			return String::from(""); 
		}
	};

	let lang = match CString::new(lang) {
		Ok(lang) => lang,
		Err(_) => {
			return String::from(""); 
		}
	};

	let c_string = unsafe { CString::from_raw(mcsql_ios_get_all(group.as_ptr(), lang.as_ptr(), auth, tech)) };
	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn metadata_get_all(lang: String) -> String {
	let lang = match CString::new(lang) {
		Ok(lang) => lang,
		Err(_) => {
			return String::from(""); 
		}
	};

	let c_string = unsafe { CString::from_raw(mcsql_metadata_get_all(lang.as_ptr())) };
	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn params_get_params() -> String {
	let c_string = unsafe { CString::from_raw(mcsql_params_get_params()) };

	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn params_get_valid_param_by_id(md_id: String) -> String {
	let md_id = match CString::new(md_id) {
		Ok(md_id) => md_id,
		Err(_) => {
			return String::from(""); 
		}
	};

	let c_string = unsafe { CString::from_raw(mcsql_params_get_valid_param_by_id(md_id.as_ptr())) };
	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn params_get_valid_param_by_group(group: String) -> String {
	let group = match CString::new(group) {
		Ok(group) => group,
		Err(_) => {
			return String::from(""); 
		}
	};

	let c_string = unsafe { CString::from_raw(mcsql_params_get_valid_param_by_group(group.as_ptr())) };
	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn ref_get_all() -> String {
	let c_string = unsafe { CString::from_raw(mcsql_ref_get_all()) };

	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn toolframe_get_all() -> String {
	let c_string = unsafe { CString::from_raw(mcsql_toolframe_get_all()) };

	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn toolframe_get_by_toolno(tool_no: i32) -> String {
	let c_string = unsafe { CString::from_raw(mcsql_toolframe_get_by_toolno(tool_no)) };

	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn userframe_get_all() -> String {
	let c_string = unsafe { CString::from_raw(mcsql_userframe_get_all()) };

	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn userframe_get_by_userno(user_no: i32) -> String {
	let c_string = unsafe { CString::from_raw(mcsql_userframe_get_by_userno(user_no)) };

	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn zeropoint_get_all() -> String {
	let c_string = unsafe { CString::from_raw(mcsql_zeropoint_get_all()) };

	match c_string.into_string() {
		Ok(s) => {return s;},
		Err(_) => {
			return String::from("");
		},
	}
}

pub fn init() {
	let conn = CString::new("/rbctrl/db/elibotDB.db").unwrap();
	unsafe {
		register_all_sql_mappers();
		mcsql_set_db_file(conn.as_ptr());
	}
}