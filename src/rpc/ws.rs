use std::sync::mpsc;
use jsonrpc_tcp_server::jsonrpc_core::*;

pub fn register_websocket_funcs(io: &mut IoHandler, websocket: mpsc::Sender<Vec<u8>>) {
	let send_to_websocket = move |message| -> bool {
		match websocket.send(value.message){
			Ok() => true,
			Err() => false,
		}
	}

	io.add_method("alarm_record_changes", |params: Params| {
		#[derive(Deserialize)]
		struct AlarmParams {
			message: 	Vec<u8>,
		}
		let value: AlarmParams = match params.parse() {
			Ok(v) => v,
			Err(_) => {
				return Ok(Value::String("fail to query".to_string()));
			},
		};

		Ok(Value::Bool(send_to_websocket(value.message)))
	});
}