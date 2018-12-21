use std::sync::mpsc;
use std::sync::Mutex;
use jsonrpc_tcp_server::jsonrpc_core::*;

pub fn register_websocket_funcs(io: &mut IoHandler, websocket: mpsc::Sender<Vec<u8>>) {
	let websocket_sender = Mutex::new(websocket);

	io.add_method("push_message_to_network", move |params: Params| {
		#[derive(Deserialize)]
		struct Message {
			message: 	String,
		}
		let value: Message = match params.parse() {
			Ok(v) => v,
			Err(_) => {
					warn!("Message parse error");
					return Ok(Value::Bool(false));
			},
		};

		match websocket_sender.lock(){
			Ok(sender) => {
				match sender.send(value.message.into_bytes()){
					Ok(()) => Ok(Value::Bool(true)),
					Err(_) => {
						warn!("Error when send to websocket");
						Ok(Value::Bool(false))
					},
				}
			}, 
			Err(_) => {
				warn!("Channel locked");
				Ok(Value::Bool(false))
			},
		}
	});
}