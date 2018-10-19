use std::thread;
use std::rc::Rc;
use std::sync::mpsc;
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
		let r = Rc::new(self.hub);

		let rc_hub = Rc::Clone(&r);
		thread::spawn(move || {
			rc_hub.run();
		});

		for request in self.server.filter_map(Result::ok) {
			let hub = Rc::Clone(&r);

			// Spawn a new thread for each connection.
			thread::spawn(move || {
				if !request.protocols().contains(&"websocket".to_string()) {
					request.reject().unwrap();
					return;
				}

				let mut conn = request.use_protocol("websocket").accept().unwrap();

				let c = client::WsClient {send: mpsc::channel(), conn: conn, hub: hub};
				hub.register.0.send(&c).unwrap();

				let ip = conn.peer_addr().unwrap();

				println!("Connection from {}", ip);

				//let (mut receiver, mut sender) = conn.split().unwrap();

				thread::spawn(move || c.write_pump());

				c.read_pump();
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