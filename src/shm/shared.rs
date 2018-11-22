use std::sync::mpsc;
use mrj_sys;

pub fn get_plc(tx: mpsc::Sender<Vec<u8>>) {
	match mrj_sys::get_plc() {
		Some(shared) => {
			let message = shared.to_bytes().iter().cloned().collect();
			tx.send(message).unwrap();
		},

		None => {},
	};
}

pub fn get_shared(tx: mpsc::Sender<Vec<u8>>) {
	match mrj_sys::get_shared() {
		Some(shared) => {
			let message = shared.to_bytes().iter().cloned().collect();
			tx.send(message).unwrap();
		},

		None => {},
	};
}

pub fn get_shared_without_check(tx: mpsc::Sender<Vec<u8>>) {
	let message = mrj_sys::get_shared_once().into_bytes();
	tx.send(message).unwrap();
}