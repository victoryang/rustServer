use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use super::client::WsClient;

pub struct Hub {
	clients: 			Vec<Arc<Mutex<WsClient>>>,
	pub register: 		mpsc::Receiver<Arc<Mutex<WsClient>>>,
	pub unregister:		mpsc::Receiver<Arc<Mutex<WsClient>>>,
	pub broadcast:		mpsc::Receiver<Vec<u8>>,
}

impl Hub {
	pub fn run(self) {
		let register = self.register;
		let unregister = self.unregister;
		let broadcast = self.broadcast;
		let clients = Arc::new(Mutex::new(self.clients));
		let cli_register = Arc::clone(&clients);
		let cli_unregister = Arc::clone(&clients);
		let cli_broadcast = Arc::clone(&clients);

		thread::spawn(move || {
			for m in register.iter().next() {
				cli_register.lock().unwrap().push(m);
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
				for c in *cli_broadcast.lock().unwrap() {
					match c.lock().unwrap().broadcast.send(m) {
						Ok(()) => print!("sending successfully"),
						Err(_) => print!("sending error"),
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