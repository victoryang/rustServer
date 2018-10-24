/* To enable macro select!*/
#![feature(mpsc_select)]

extern crate websocket;

extern crate timer;
extern crate chrono;
extern crate fern;

//#[macro_use]
extern crate log;

use std::sync::mpsc;

mod daemon;
mod shm;
mod ws;
mod rlog;

fn setup_log() {
	/*if rlog::check_file_size_exceeded_max(&filename) {
		let backupfilename = filename.push_str(".bak");
		fs::rename(&filename, backupfilename);
	}*/

	rlog::setup_logging(1, "/rbctrl/apiserver/log/rust.log".to_string()).expect("Failed to initialize logging.");
}

fn main() {
	setup_log();

	info!("starting websocket server...");
	let (websocket_tx, websocket_rx) = mpsc::channel::<Vec<u8>>();
	let wss = ws::new_websocket_server("0.0.0.0:9050");
	wss.run(websocket_rx);

	let shmserver = shm::new_shm_server(websocket_tx.clone());
	shmserver.init().run();
    //daemon::Run();
}
