use std::thread;
use std::sync::mpsc;
use jsonrpc_tcp_server::*;
use jsonrpc_tcp_server::jsonrpc_core::*;

mod mcsql;
mod mrj;

pub struct RpcServer {
}

impl RpcServer {
	pub fn run(&self, websocket_tx: mpsc::Sender<Vec<u8>>) {
		thread::spawn(|| {
			let mut io = IoHandler::default();
			register_method(&mut io);
			register_websocket_method(&mut io, websocket_tx);

			let server = ServerBuilder::new(io)
						.start(&"0.0.0.0:9030".parse().unwrap())
						.expect("Server must start with no issues");

			server.wait();
		});
	}
}

pub fn new_rpc_server() -> RpcServer {
	mcsql_sys::init();

	RpcServer {
	}
}

fn register_method(io: &mut IoHandler) {
	mcsql::register_mcsql_funcs(io);
	mrj::register_mcsql_funcs(io);
}

fn register_websocket_method(io: &mut IoHandler, websocket: mpsc::Sender<Vec<u8>>) {
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

		websocket.send(value.message).unwrap();
		Ok(Value::Bool(true))
	});
}