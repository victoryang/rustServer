use std::sync::mpsc;
use std::sync::Mutex;
use jsonrpc_tcp_server::jsonrpc_core::*;

pub fn register_websocket_funcs(io: &mut IoHandler, websocket: mpsc::Sender<Vec<u8>>) {
	let websocket_sender = Mutex::new(websocket);

	io.add_method("alarm_record_changes", move |params: Params| {
		#[derive(Deserialize)]
		struct AlarmParams {
			message: 	Vec<u8>,
		}
		let value: AlarmParams = match params.parse() {
			Ok(v) => v,
			Err(_) => {
				return Ok(Value::Bool(false));
			},
		};

		match websocket_sender.lock(){
			Ok(sender) => {
				match sender.send(value.message){
					Ok(()) => Ok(Value::Bool(true)),
					Err(_) => Ok(Value::Bool(false)),
				}
			}, 
			Err(_) => Ok(Value::Bool(false)),
		}
	});
}