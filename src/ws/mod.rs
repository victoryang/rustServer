use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use websocket::sync::Server;
use websocket::server::NoTlsAcceptor;

mod hub;
mod client;

pub struct WsServer<'a> {
	addr: 		String,
	server: 	Server<NoTlsAcceptor>,
	register:	mpsc::Sender<&'a WsClient>,
	unregister:	mpsc::Sender<&'a WsClient>,
	broadcast: 	mpsc::Sender<Vec<u8>>,
	hub:		hub::Hub,
}

impl WsServer {
	pub fn run(self) {
		self.hub.run();

		for request in self.server.filter_map(Result::ok) {
			let register = self.register.clone();
			let unregister = self.unregister.clone();

			// Spawn a new thread for each connection.
			thread::spawn(move || {
				if !request.protocols().contains(&"websocket".to_string()) {
					request.reject().unwrap();
					return;
				}

				let conn = request.use_protocol("websocket").accept().unwrap();

				let ip = conn.peer_addr().unwrap();
				println!("Connection from {}", ip);

				let (broadcast_sender, broadcast_receiver) = mpsc::channel();
				let c = client::WsClient {conn: conn, broadcast: broadcast_sender};
				let arc_c = &c;

				register.send(arc_c).unwrap();

				arc_c.run(broadcast_receiver);

				unregister.send(arc_c).unwrap();
			});
		}
	}

	pub fn broadcast(&self, msg: Vec<u8>) {
		self.broadcast.send(msg);
	}
}

pub fn new_websocket_server(addr: &str) -> WsServer {
	let (register_sender, register_receiver) = mpsc::channel();
	let (unregister_sender, unregister_receiver) = mpsc::channel();
	let (broadcast_sender, broadcast_receiver) = mpsc::channel();

	WsServer {
		addr: 		addr.to_string(),
		server: 	Server::bind(addr).unwrap(),
		register: 	register_sender,
		unregister: unregister_sender,
		broadcast: 	broadcast_sender,
		hub:		hub::new_hub(register_receiver, unregister_receiver, broadcast_receiver),
	}
}