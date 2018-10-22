use std::thread;
use std::sync::mpsc;
use websocket::sync::Server;
use websocket::server::NoTlsAcceptor;

mod hub;
mod client;

let ws_hub = hub::new_hub();

pub struct WsServer {
	addr: 	String,
	server: Server<NoTlsAcceptor>,
}

impl WsServer {
	pub fn run(&self) {
		ws_hub.run();
		let server = self.server;

		for request in server.filter_map(Result::ok) {
			let register = ws_hub.register.0.clone();
			let unregister = ws_hub.unregister.0.clone();
			// Spawn a new thread for each connection.
			thread::spawn(move || {
				if !request.protocols().contains(&"websocket".to_string()) {
					request.reject().unwrap();
					return;
				}

				let conn = request.use_protocol("websocket").accept().unwrap();

				let c = client::WsClient {send: mpsc::channel(), conn: conn, unregister: unregister};
				register.send(c).unwrap();

				let ip = conn.peer_addr().unwrap();

				println!("Connection from {}", ip);

				//let (mut receiver, mut sender) = conn.split().unwrap();

				thread::spawn(move || c.write_pump());

				c.read_pump();
			});
		}
	}

	pub fn broadcast(&self, msg: Vec<u8>) {
		ws_hub.broadcast(msg);
	}
}

pub fn new_websocket_server(addr: &str) -> WsServer {
	WsServer {
		addr: 	addr.to_string(),
		server: Server::bind(addr).unwrap(),
	}
}