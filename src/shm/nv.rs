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

pub fn get_nv_without_check(tx: mpsc::Sender<Vec<u8>>) {
	let message = mrj_sys::get_nv_once().into_bytes();
	tx.send(message).unwrap();
}