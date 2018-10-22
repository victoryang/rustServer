use websocket::sync::Client;
use std::net::TcpStream;
use std::sync::mpsc;
use websocket::message::OwnedMessage;

pub struct WsClient {
	pub send: 			(mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>),
	pub conn:			Client<TcpStream>,
	pub unregister: 	mpsc::Sender<WsClient>,
}

impl WsClient {
	pub fn write_pump(&self) {
		let (_, mut sender) = self.conn.split().unwrap();
		let iter = self.send.1.iter();
		for m in iter.next() {
			match m {
				_ => {
					let message = OwnedMessage::Binary(m);
					sender.send_message(&message).unwrap();
				},
			}
		}
	}

	pub fn read_pump(&self) {
		let (mut receiver, mut sender) = self.conn.split().unwrap();
		for message in receiver.incoming_messages() {
			let message = message.unwrap();

			match message {
				OwnedMessage::Close(_) => {
					println!("Client disconnected");
					self.unregister.send(*self).unwrap();
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