use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use websocket::sync::Server;
use websocket::server::NoTlsAcceptor;

mod client;
mod dispatcher;

pub struct WsServer {
	server: 	Server<NoTlsAcceptor>,
}

impl WsServer {
	pub fn run(self, dispatcher_rx: mpsc::Receiver<Vec<u8>>) {
		let client_senders: Arc<Mutex<Vec<mpsc::Sender<Vec<u8>>>>> = Arc::new(Mutex::new(vec![]));

		let client_senders = client_senders.clone();

		let dp = dispatcher::Dispatcher{receiver: dispatcher_rx};
		{
			let client_senders = client_senders.clone();
			dp.dispatch(client_senders);
		}

		for request in self.server.filter_map(Result::ok) {
			let (client_tx, client_rx) = mpsc::channel::<Vec<u8>>();
			client_senders.lock().unwrap().push(client_tx);

			// Spawn a new thread for each connection.
			thread::spawn(move || {
				if !request.protocols().contains(&"websocket".to_string()) {
					request.reject().unwrap();
					return;
				}

				let conn = request.use_protocol("websocket").accept().unwrap();

				let ip = conn.peer_addr().unwrap();
				println!("Connection from {}", ip);

				let c = client::WsClient {conn: conn, receiver: client_rx};
				c.run();
			});
		}
	}
}

pub fn new_websocket_server(addr: &str) -> WsServer {
	WsServer {
		server: 	Server::bind(addr).unwrap(),
	}
}