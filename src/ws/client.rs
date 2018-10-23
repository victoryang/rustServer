use websocket::sync::Client;
use std::net::TcpStream;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use websocket::message::OwnedMessage;

pub struct WsClient {
	pub conn:			Client<TcpStream>,
	pub broadcast:		mpsc::Sender<Vec<u8>>,
}

impl WsClient {
	pub fn run(self, broadcast_receiver: mpsc::Receiver<Vec<u8>>) {
		let (receiver, sender) = self.conn.split().unwrap();
		let sstream = Arc::new(Mutex::new(sender));
		let rstream = Arc::new(Mutex::new(receiver));

		thread::spawn(move || {
			let iter = broadcast_receiver.iter();
			for m in iter.next() {
				match m {
					_ => {
						let message = OwnedMessage::Binary(m);
						sstream.lock().unwrap().send_message(&message).unwrap();
					},
				}
			}
		});

		let read_pump = thread::spawn(move || {
			for message in rstream.lock().unwrap().incoming_messages() {
				let message = message.unwrap();

				match message {
					OwnedMessage::Close(_) => {
						println!("Client disconnected");
						return;
					}
					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
						sstream.lock().unwrap().send_message(&message).unwrap();
					}
					_ => println!("Receive new message from client, drop it."),
				}
			}
		});

		read_pump.join();
	}
}