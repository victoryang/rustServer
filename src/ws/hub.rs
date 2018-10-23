use std::thread;
use std::sync::mpsc;
use std::sync::Mutex;

use super::client::WsClient;

pub struct Hub {
	clients: 			Vec<WsClient>,
	pub register: 		mpsc::Receiver<WsClient>,
	pub unregister:		mpsc::Receiver<WsClient>,
	pub broadcast:		mpsc::Receiver<Vec<u8>>,
}

impl Hub {
	pub fn run(self) {
		let register = mutex::new(self.register);
		let unregister = mutex::new(self.unregister);
		let broadcast = mutex::new(self.broadcast);
		let clients = mutex::new(self.clients);
		thread::spawn(move || {
			let iter = register.lock().unwrap().iter();
			for m in iter.next() {
				clients.lock().unwrap().push(m);
			};
		});
		thread::spawn(move || {
			let iter = unregister.lock().unwrap().iter();
			for m in iter.next() {
				match m {
					_ => print!("remove client from hub")
				}
			};
		});
		thread::spawn(move || {
			let iter = broadcast.lock().unwrap().iter();
			for m in iter.next() {
				match m {
					_ => {for c in &clients {c.broadcast.send(m).unwrap();}},
				}
			};
		});
	}
}

pub fn new_hub(register: mpsc::Receiver<WsClient>, 
			unregister: mpsc::Receiver<WsClient>,
			broadcast: mpsc::Receiver<Vec<u8>>) -> Hub {
	Hub {
		clients: 	Vec::new(),
		register,
		unregister,
		broadcast,
	}
}