use std::thread;
use std::sync::Mutex;
use std::sync::mpsc;
use websocket::sync::Server;
use websocket::server::NoTlsAcceptor;

mod hub;
mod client;

pub struct WsServer {
	addr: 	String,
	server: Server<NoTlsAcceptor>,
	hub:	hub::Hub,
}

impl WsServer {
	pub fn run(self) {
		self.hub.run();

		for request in self.server.filter_map(Result::ok) {
			let register = self.hub.register.0.clone();
			let unregister = self.hub.unregister.0.clone();

			// Spawn a new thread for each connection.
			thread::spawn(move || {
				if !request.protocols().contains(&"websocket".to_string()) {
					request.reject().unwrap();
					return;
				}

				let conn = request.use_protocol("websocket").accept().unwrap();

				let ip = conn.peer_addr().unwrap();
				println!("Connection from {}", ip);

				let (broadcast_sender, broadcast_receiver)  = mpsc::channel();
				let c = client::WsClient {conn: conn, broadcast: broadcast_sender};

				let cli_mux = Mutex::new(c);
				register.send(cli_mux).unwrap();

				*cli_mux.write_pump(broadcast_receiver);

				*cli_mux.read_pump();

				unregister.send(cli_mux).unwrap();
			});
		}
	}

	pub fn broadcast(&self, msg: Vec<u8>) {
		self.hub.broadcast(msg);
	}
}

pub fn new_websocket_server(addr: &str) -> WsServer {
	WsServer {
		addr: 	addr.to_string(),
		server: Server::bind(addr).unwrap(),
		hub:	hub::new_hub(),
	}
}