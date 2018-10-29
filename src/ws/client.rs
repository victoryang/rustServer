use websocket::sync::Client;
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc;
use websocket::message::OwnedMessage;

pub struct WsClient {
	pub conn:			Client<TcpStream>,
}

impl WsClient {
	pub fn run(self, receiver: mpsc::Receiver<Vec<u8>>) {
		let (mut rstream, mut sstream) = self.conn.split().unwrap();
		let(tx, rx) = mpsc::channel::<OwnedMessage>();
		let tx_receiver = tx.clone();

		let _ = thread::spawn(move || {
			loop {
				let message = match rx.recv() {
					Ok(m) => m,
					Err(e) => {
						println!("Send Loop: {:?}", e);
						return;
					}
				};

				match message {
					OwnedMessage::Close(_) => {
						println!("Client disconnected");
						return;
					},

					OwnedMessage::Pong(ping) => {
						let message = OwnedMessage::Pong(ping);
						sstream.send_message(&message).unwrap();
					},

					_ => {
						match sstream.send_message(&message) {
							Ok(()) => (),
							Err(e) => {
								warn!("sending messages to channel error: {:?}", e);
								let _ = sstream.send_message(OwnedMessage::Close(_));
								return;
							}
						}
					},
				}
			}
		});

		let _ = thread::spawn(move || {
			for message in rstream.incoming_messages() {
				let message = match message {
					Ok(m) => m,
					Err(e) => {
						warn!("read error from channel: {:?}", e);
						return;
					}
				}

				match message {
					OwnedMessage::Close(_) => {
						tx_receiver.send(message).unwrap();
						println!("Client disconnected");
						return;
					}
					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
						tx_receiver.send(message).unwrap();
					}
					_ => println!("Receive new message from client, drop it."),
				}
			}
		});

		let mut iter = receiver.iter();
		loop {
			match iter.next() {
				Some(message) => {
					let message = OwnedMessage::Binary(message);
					tx.send(message).unwrap();
				},
				None => break,
			}
		}
	}
}