use std::thread;
use std::sync::mpsc;

use super::client::WsClient;

pub struct Hub {
	clients: 		Vec<WsClient>,
	pub register: 		(mpsc::Sender<WsClient>, mpsc::Receiver<WsClient>),
	pub unregister:		(mpsc::Sender<WsClient>, mpsc::Receiver<WsClient>),
	pub broadcast:		(mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>),
}

impl Hub {
	pub fn run(&self) {
		let register = self.register.1;
		let unregister = self.unregister.1;
		let broadcast = self.broadcast.1;
		thread::spawn(move || {for m in register.recv().unwrap() {println!("{}", m);};});
		thread::spawn(move || {for m in unregister.recv().unwrap() {println!("{}", m);};});
		thread::spawn(move || {for m in broadcast.recv().unwrap() {println!("{}", m);};});
		/*loop {
			select! {
				c = self.register.1.recv().unwrap() => {self.clients.push(c);},
				c = self.unregister.1.recv().unwrap() => {self.clients.remove_item(c);},
				m = self.broadcast.1.recv().unwrap() => {for c in &self.clients {c.send.0.send(m).unwrap();}},
			}
		}*/
	}

	pub fn broadcast(&self, msg: Vec<u8>) {
		self.broadcast.0.send(msg).unwrap();
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