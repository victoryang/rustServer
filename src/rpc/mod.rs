use std::thread;
use jsonrpc_tcp_server::*;
use jsonrpc_tcp_server::jsonrpc_core::*;

use mcsql_sys;

pub struct RpcServer {
}

impl RpcServer {
	pub fn run(&self) {
		thread::spawn(|| {
			let mut io = IoHandler::default();
			register_method(&mut io);

			let server = ServerBuilder::new(io)
						.start(&"0.0.0.0:9030".parse().unwrap())
						.expect("Server must start with no issues");

			server.wait();
		});
	}
}

pub fn new_rpc_server() -> RpcServer {
	RpcServer {
	}
}

fn register_method(io: &mut IoHandler) {
	io.add_method("arc_get_all", |_params: Params| {
		let res = mcsql_sys::arc_get_all();
		Ok(Value::String(res))
	});
}