use std::sync::mpsc;
use std::thread;

pub struct Dispatcher{
	receiver: mpsc::Receiver<Vec<u8>>
}

impl Dispatcher {
	pub fn dispatch(self, client_senders: Arc<Mutex<Vec<mpsc::Sender<Vec<u8>>>>>) {
		let receiver = self.receiver;

		thread::spawn(move || {
			while let Ok(msg) = receiver.recv() {
				let mut removed_clients = Vec::new();
				for (i, sender) in client_senders.lock().unwrap().iter().enumerate() {
					match sender.send(msg.clone()) {
						Ok(()) => {},
						Err(_) => {
							removed_clients.push(i);
						}
					}
				}

				let mut removed_nbr = 0;
				for i in removed_clients {
					client_senders.lock().unwrap().swap_remove(i-removed_nbr);
					removed_nbr +=1;
				}
			}
		});
	}
}