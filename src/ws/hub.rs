use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use super::client::WsClient;

pub struct Hub {
	clients: 			Vec<&WsClient>,
	pub register: 		mpsc::Receiver<&WsClient>,
	pub unregister:		mpsc::Receiver<&WsClient>,
	pub broadcast:		mpsc::Receiver<Vec<u8>>,
}

impl Hub {
	pub fn run(self) {
		let register = self.register;
		let unregister = self.unregister;
		let broadcast = self.broadcast;
		let clients = Arc::new(Mutex::new(self.clients));

		thread::spawn(move || {
			for c in register.iter().next() {
				clients.lock().unwrap().push(c);
			};
		});

		thread::spawn(move || {
			for c in unregister.iter().next() {
				print!("remove client from hub");
				//clients.lock().unwrap().remove(c);
			};
		});

		thread::spawn(move || {
			for m in broadcast.iter().next() {
				for c in clients.lock().unwrap() {
					match c.send(m) {
						Ok() => print!("sending successfully"),
						Err() => print!("sending error"),
					};
				}
			};
		});
	}
}

pub fn new_hub(register: mpsc::Receiver<Arc<Mutex<WsClient>>>, 
			unregister: mpsc::Receiver<Arc<Mutex<WsClient>>>,
			broadcast: mpsc::Receiver<Vec<u8>>) -> Hub {
	Hub {
		clients: 	Vec::new(),
		register,
		unregister,
		broadcast,
	}
}