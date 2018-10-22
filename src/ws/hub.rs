use std::thread;
use std::sync::mpsc;
use std::sync::Mutex;

use super::client::WsClient;

pub struct Hub {
	clients: 			Vec<WsClient>,
	pub register: 		(mpsc::Sender<Mutex<WsClient>>, mpsc::Receiver<Mutex<WsClient>>),
	pub unregister:		(mpsc::Sender<Mutex<WsClient>>, mpsc::Receiver<Mutex<WsClient>>),
	pub broadcast:		(mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>),
}

impl Hub {
	pub fn run(self) {
		let register = self.register.1;
		let unregister = self.unregister.1;
		let broadcast = self.broadcast.1;
		let clients = self.clients;
		thread::spawn(move || {
			let iter = register.iter();
			for m in iter.next() {
				match m.lock().unwrap() {
					c => clients.push_back(c),
				}
			};
		});
		thread::spawn(move || {
			let iter = unregister.iter();
			for m in iter.next() {
				match m {
					_ => print!("remove client from hub")
				}
			};
		});
		thread::spawn(move || {
			let iter = broadcast.iter();
			for m in iter.next() {
				match m {
					_ => {for c in &clients {c.broadcast.send(m).unwrap();}},
				}
			};
		});
	}

	pub fn broadcast(&self, msg: Vec<u8>) {
		self.broadcast.0.send(msg);
	}
}

pub fn new_hub() -> Hub {
	Hub {
		clients: 	Vec::new(),
		register: 	mpsc::channel(),
		unregister: mpsc::channel(),
		broadcast:	mpsc::channel(),
	}
}