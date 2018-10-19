use websocket::async::Client;
use std::sync::mpsc;
use websocket::message::OwnedMessage;

use hub::Hub;

struct WsClient {
	send: 	(mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>),
	conn:	Client,
	hub: 	Hub,
}

impl WsClient {
	pub fn write_pump(&self) {
		let (_, mut sender) = self.conn.split().unwrap();
		for m in client.send.1.recv().unwrap() {
			let message = OwnedMessage::Binary(m);
			sender.send_message(&message).unwrap();
		}
	}

	pub fn read_pump(&self) {
		let (mut receiver, mut sender) = self.conn.split().unwrap();
		for message in receiver.incoming_messages() {
			let message = message.unwrap();

			match message {
				OwnedMessage::Close(_) => {
					println!("Client {} disconnected", ip);
					self.hub.unregister.0.send(self).unwrap();
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