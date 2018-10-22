use websocket::sync::Client;
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc;
use websocket::message::OwnedMessage;

pub struct WsClient {
	pub conn:			Client<TcpStream>,
	pub broadcast:		mpsc::Sender<Vec<u8>>,
}

impl WsClient {
	pub fn write_pump(&self, receiver: mpsc::Receiver<Vec<u8>>) {
		let (_, mut sender) = self.conn.split().unwrap();
		thread::spawn(move || {
			let iter = receiver.iter();
			for m in iter.next() {
				match m {
					_ => {
						let message = OwnedMessage::Binary(m);
						sender.send_message(&message).unwrap();
					},
				}
			}
		});
	}

	pub fn read_pump(&self) {
		let (mut receiver, mut sender) = self.conn.split().unwrap();
		for message in receiver.incoming_messages() {
			let message = message.unwrap();

			match message {
				OwnedMessage::Close(_) => {
					println!("Client disconnected");
					return;
				}
				OwnedMessage::Ping(ping) => {
					let message = OwnedMessage::Pong(ping);
					sender.send_message(&message).unwrap();
				}
				_ => println!("Receive new message from client, drop it."),
			}
		}
	}
}