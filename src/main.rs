extern crate websocket;

extern crate chrono;
extern crate fern;
#[macro_use]
extern crate log;

mod daemon;
mod shm;
mod ws;
mod rlog;

fn setup_log() {
	/*if rlog::check_file_size_exceeded_max(&filename) {
		let backupfilename = filename.push_str(".bak");
		fs::rename(&filename, backupfilename);
	}*/

	rlog::setup_logging(1, "/rbctrl/apiserver/log/rust.log").expect("Failed to initialize logging.");
}

/*fn setup_shm_environment() {
	info!("setup_shm_environment");
	shm::new_shm_server();
}*/

fn main() {
	setup_log();
	//setup_shm_environment();
	let wss = ws::new_websocket_server("0.0.0.0:9050");
	wss.run();
    //daemon::Run();
}
