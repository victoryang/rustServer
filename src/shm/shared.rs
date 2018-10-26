use std::sync::mpsc;
use mrj_sys;

pub fn get_plc(tx: mpsc::Sender<Vec<u8>>) {
	
}

pub fn get_shared(tx: mpsc::Sender<Vec<u8>>) {
	match mrj_sys::get_shared() {
		Option(shared) => {
			let message = shared.to_bytes().iter().cloned().collect();
			tx.send(message).unwrap();
		},

		None => {},
	};
}