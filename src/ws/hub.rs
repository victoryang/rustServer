use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use super::client::WsClient;

pub struct Hub {
	clients: 			Vec<WsClient>,
	pub register: 		mpsc::Receiver<WsClient>,
	pub unregister:		mpsc::Receiver<WsClient>,
	pub broadcast:		mpsc::Receiver<Vec<u8>>,
}

impl Hub {
	pub fn run(self) {
		let register = /*Arc::new(Mutex::new(self.register));*/self.register;
		let unregister = /*Arc::new(Mutex::new(self.unregister));*/self.unregister;
		let broadcast = /*Arc::new(Mutex::new(self.broadcast));*/self.broadcast;
		let clients = Arc::new(Mutex::new(self.clients));
		thread::spawn(move || {
			/*let iter = register.lock().unwrap().iter();*/
			for m in register.iter().next() {
				let c = m.lock().unwrap();
				clients.lock().unwrap().push(c);
			};
		});
		thread::spawn(move || {
			/*let iter = unregister.lock().unwrap().iter();*/
			for m in unregister.iter().next() {
				match m {
					_ => print!("remove client from hub")
				}
			};
		});
		thread::spawn(move || {
			/*let iter = broadcast.lock().unwrap().iter();*/
			for m in broadcast.iter().next() {
				for c in &clients.lock().unwrap() {
					c.broadcast.send(m).unwrap();
				}
				/*match m {
					_ => {for c in &clients {c.broadcast.send(m).unwrap();}},
				}*/
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