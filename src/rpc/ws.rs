use std::sync::mpsc;
use jsonrpc_tcp_server::jsonrpc_core::*;

pub fn register_websocket_funcs(io: &mut IoHandler, websocket: mpsc::Sender<Vec<u8>>) {
	io.add_method("alarm_record_changes", move |params: Params| {
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

		let websocket_sender = websocket.clone();

		match websocket_sender.send(value.message){
			Ok(()) => Ok(Value::Bool(true)),
			Err(_) => Ok(Value::Bool(false)),
		}
	});
}