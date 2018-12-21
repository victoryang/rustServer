use jsonrpc_tcp_server::jsonrpc_core::*;

use mrj_sys;

pub fn register_mrj_funcs(io: &mut IoHandler) {
	io.add_method("get_system_variables", |params: Params| {
		#[derive(Deserialize)]
		struct SysVarParams {
			datatype:	i32,
			start:		i32,
			end:		i32,
		}
		let value: SysVarParams = match params.parse() {
			Ok(v) => v,
			Err(_) => {
				return Ok(Value::String("fail to query".to_string()));
			},
		};

		let res = mrj_sys::get_system_variables(value.datatype, value.start, value.end);
		Ok(Value::String(res))
	});

	io.add_method("get_local_variables", |params: Params| {
		#[derive(Deserialize)]
		struct LocVarParams {
			datatype:	i32,
			num:		i32,
			start:		i32,
			end:		i32,
		}
		let value: LocVarParams = match params.parse() {
			Ok(v) => v,
			Err(_) => {
				return Ok(Value::String("fail to query".to_string()));
			},
		};

		let res = mrj_sys::get_local_variables(value.datatype, value.num, value.start, value.end);
		Ok(Value::String(res))
	});

	io.add_method("get_plc_once", |_params: Params| {
		let res = mrj_sys::get_plc_once();
		Ok(Value::String(res))
	});

	io.add_method("get_shared_once", |_params: Params| {
		let res = mrj_sys::get_shared_once();
		Ok(Value::String(res))
	});

	io.add_method("get_nv_once", |_params: Params| {
		let res = mrj_sys::get_nv_once();
		Ok(Value::String(res))
	});

	io.add_method("get_remote_mode_status", |_params: Params| {
		let res = mrj_sys::get_remote_mode_status();
		Ok(Value::Number(serde_json::value::Number::from(res)))
	});

	io.add_method("get_encryption_status", |_params: Params| {
		let res = mrj_sys::get_encryption_status();
		Ok(Value::Number(serde_json::value::Number::from(res)))
	});

	io.add_method("get_encryption_remain_time", |_params: Params| {
		let res = mrj_sys::get_encryption_remain_time();
		Ok(Value::Number(serde_json::value::Number::from(res)))
	});

	io.add_method("get_machine_code", |_params: Params| {
		let res = mrj_sys::get_machine_code();
		Ok(Value::String(res))
	});
}