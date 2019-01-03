use websocket::sync::Client;
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc;
use websocket::{Message, OwnedMessage};

pub struct WsClient {
	pub conn:			Client<TcpStream>,
}

impl WsClient {
	pub fn run(self, receiver: mpsc::Receiver<Vec<u8>>) {
		let (mut rstream, mut sstream) = self.conn.split().unwrap();
		let(tx, rx) = mpsc::channel::<OwnedMessage>();
		let tx1 = mpsc::Sender::clone(&tx);

		let send_loop = thread::spawn(move || {
			loop {
				let message = match rx.recv() {
					Ok(m) => m,
					Err(e) => {
						warn!("Send Loop: {:?}", e);
						return;
					}
				};

				match message {
					OwnedMessage::Close(_) => {
						debug!("Client disconnected");
						return;
					},

					OwnedMessage::Pong(ping) => {
						let message = OwnedMessage::Pong(ping);
						match sstream.send_message(&message) {
							Ok(()) => (),
							Err(e) => {
								warn!("Sending Pong messages to network error: {:?}", e);
								let _ = sstream.send_message(&Message::close());
								return;
							}
						}
					},

					_ => {
						match sstream.send_message(&message) {
							Ok(()) => (),
							Err(e) => {
								warn!("Sending messages to network error: {:?}", e);
								let _ = sstream.send_message(&Message::close());
								return;
							}
						}
					},
				}
			}
		});

		let receive_loop = thread::spawn(move || {
			for message in rstream.incoming_messages() {
				let message = match message {
					Ok(m) => m,
					Err(e) => {
						warn!("Read error from network: {:?}", e);
						let _ = tx1.send(OwnedMessage::Close(None));
						return;
					}
				};

				match message {
					OwnedMessage::Close(_) => {
						match tx1.send(message) {
							Ok(()) => (),
							Err(e) => {
								warn!("Sending Close to sstream error: {:?}", e);
								return;
							}
						}
						debug!("Client disconnected");
						return;
					}

					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
						match tx1.send(message) {
							Ok(()) => (),
							Err(e) => {
								warn!("Sending Ping to sstream error: {:?}", e);
								return;
							}
						}
					}

					_ => debug!("Received new message from client, drop it."),
				}
			}
		});

		let _ = thread::spawn(move || {
			let mut iter = receiver.iter();
			loop {
				match iter.next() {
					Some(message) => {
						let message = OwnedMessage::Binary(message);
						match tx.send(message) {
							Ok(()) => (),
							Err(e) => {
								debug!("Sending messages to sstream error: {:?}", e);
								return;
							}
						}
					},
					None => break,
				}
			}
		});
		

		let _ = send_loop.join();
		let _ = receive_loop.join();

		debug!("Client exited");
	}
}