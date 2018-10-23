use websocket::sync::Client;
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc;
use websocket::message::OwnedMessage;

pub struct WsClient {
	pub conn:			Client<TcpStream>,
	pub dispatcher:		mpsc::Receiver<Vec<u8>>,
}

impl WsClient {
	pub fn run(self) {
		let (receiver, sender) = self.conn.split().unwrap();
		let(tx, rx) = mpsc::channel::<OwnedMessage>();

		let _ = thread::spawn(move || {
			for message in rx.try_recv() {
				match message {
					OwnedMessage::Close(_) => {
						println!("Client disconnected");
						return;
					},

					OwnedMessage::Pong(ping) => {
						let message = OwnedMessage::Pong(ping);
						sender.send_message(&message).unwrap();
					},

					_ => {
						sender.send_message(&message).unwrap();
					},
				}
			}
		});

		let _ = thread::spawn(move || {
			for message in receiver.incoming_messages() {
				let message = message.unwrap();

				match message {
					OwnedMessage::Close(_) => {
						tx.send(&message).unwrap();
						println!("Client disconnected");
						return;
					}
					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
						tx.send(&message).unwrap();
					}
					_ => println!("Receive new message from client, drop it."),
				}
			}
		});

		for message in self.dispatcher.try_recv() {
			let message = OwnedMessage::Binary(message);
			tx.send(&message);
		}
	}
}