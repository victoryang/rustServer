use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use websocket::sync::Server;
use websocket::server::NoTlsAcceptor;

mod client;
mod dispatcher;

pub struct WsServer {
	addr: 		String,
	server: 	Server<NoTlsAcceptor>,
	broadcast:	mpsc::Sender<Vec<u8>>,
}

impl WsServer {
	pub fn run(&self) {
		let (dispatcher_tx, dispatcher_rx) = mpsc::channel::<Vec<u8>>();
		self.broadcast = dispatcher_tx;

		let client_senders: Arc<Mutex<Vec<mpsc::Sender<Vec<u8>>>>> = Arc::new(Mutex::new(vec![]));

		{
			let client_senders = client_senders.clone();
			let dp = dispatcher::Dispatcher {
				receiver:	dispatcher_rx,
			};
			dp.dispatch(client_senders);
		}

		for request in self.server.filter_map(Result::ok) {
			let (client_tx, client_rx) = mpsc::channel<Vec<u8>>();
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

				let c = client::WsClient {conn: conn, dispatch: client_rx};
				c.run();
			});
		}
	}

	pub fn broadcast(&self, msg: Vec<u8>) {
		self.broadcast.send(msg);
	}
}

pub fn new_websocket_server(addr: &str) -> WsServer {
	let (dispatcher, _) = mpsc::channel<Vec<u8>>();
	WsServer {
		addr: 		addr.to_string(),
		server: 	Server::bind(addr).unwrap(),
		broadcast:	dispatcher,
	}
}