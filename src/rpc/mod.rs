use std::thread;
use std::sync::mpsc;
use jsonrpc_tcp_server::*;
use jsonrpc_tcp_server::jsonrpc_core::*;

mod mcsql;
mod mrj;
mod ws;

pub struct RpcServer {
}

impl RpcServer {
	pub fn run(&self, websocket: mpsc::Sender<Vec<u8>>) {
		thread::spawn(move || {
			let mut io = IoHandler::default();
			register_method(&mut io);
			ws::register_websocket_funcs(&mut io, websocket);

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