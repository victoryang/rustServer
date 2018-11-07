use jsonrpc_tcp_server::*;
use jsonrpc_tcp_server::jsonrpc_core::*;

use mcsql_sys;

pub fn register_mcsql_funcs(io: &mut IoHandler) {
	io.add_method("arc_get_all", |_params: Params| {
		let res = mcsql_sys::arc_get_all();
		Ok(Value::String(res))
	});

	io.add_method("bookprogram_get_all", |_params: Params| {
		let res = mcsql_sys::bookprogram_get_all();
		Ok(Value::String(res))
	});

	io.add_method("enum_get_all", |_params: Params| {
		let res = mcsql_sys::enum_get_all();
		Ok(Value::String(res))
	});

	io.add_method("extaxis_get_all", |_params: Params| {
		let res = mcsql_sys::extaxis_get_all();
		Ok(Value::String(res))
	});

	io.add_method("interference_get_all", |_params: Params| {
		let res = mcsql_sys::interference_get_all();
		Ok(Value::String(res))
	});

	io.add_method("params_get_params", |_params: Params| {
		let res = mcsql_sys::params_get_params();
		Ok(Value::String(res))
	});

	io.add_method("ref_get_all", |_params: Params| {
		let res = mcsql_sys::ref_get_all();
		Ok(Value::String(res))
	});

	io.add_method("toolframe_get_all", |_params: Params| {
		let res = mcsql_sys::toolframe_get_all();
		Ok(Value::String(res))
	});

	io.add_method("userframe_get_all", |_params: Params| {
		let res = mcsql_sys::userframe_get_all();
		Ok(Value::String(res))
	});

	io.add_method("zeropoint_get_all", |_params: Params| {
		let res = mcsql_sys::zeropoint_get_all();
		Ok(Value::String(res))
	});
}