use std::sync::mpsc;
use mrj_sys;

pub fn get_nv(tx: mpsc::Sender<Vec<u8>>) {
	match mrj_sys::get_nv() {
		Some(shared) => {
			let message = shared.to_bytes().iter().cloned().collect();
			tx.send(message).unwrap();
		},

		None => {},
	};
}