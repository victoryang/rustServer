use std::thread;
use websocket::sync::Server;

mod hub;
mod client;

pub struct WsServer {
	addr: 	str,
	server: Server,
	hub: 	hub::Hub,
}

impl WsServer {
	pub fn run(&self) {
		for request in self.server.filter_map(Result::ok) {
			// Spawn a new thread for each connection.
			thread::spawn(move || {
				if !request.protocols().contains(&"rust-websocket".to_string()) {
					request.reject().unwrap();
					return;
				}

				let mut conn = request.use_protocol("rust-websocket").accept().unwrap();

				let cli = client::WsClient {send: mpsc::channel(), conn: conn, hub: self.hub};
				self.hub.register.0.send(&cli).unwrap();

				let ip = conn.peer_addr().unwrap();

				println!("Connection from {}", ip);

				//let (mut receiver, mut sender) = conn.split().unwrap();

				thread::spawn(move || cli.write_pump());

				cli.read_pump();
			});
		}
	}

	pub fn broadcast(&self, msg: [u8]) {
		self.hub.broadcast(msg);
	}
}

pub fn new_websocket_server(addr: &str) -> WsServer {
	WsServer {
		addr: 	addr,
		server: Server::bind(addr).unwrap(),
		hub:	new_hub(),
	}
}