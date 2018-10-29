/* To enable macro select!*/
#![feature(mpsc_select)]

extern crate websocket;

extern crate timer;
extern crate chrono;
extern crate fern;

#[cfg(unix)]
extern crate signal;
extern crate nix;

#[macro_use]
extern crate log;

extern crate mrj_sys;

use std::sync::mpsc;
use std::thread;

use signal::trap::Trap;
use nix::sys::signal::{SIGUSR1, SIGHUP, SIGINT, SIGTERM};

mod shm;
mod ws;
mod rlog;

fn handle_signals() {
    let trap = Trap::trap(&[SIGTERM, SIGINT, SIGHUP, SIGUSR1]);
    for sig in trap {
        match sig {
            SIGTERM | SIGINT | SIGHUP => {
                info!("receive signal {}, stopping server...", sig);
                break;
            }
            SIGUSR1 => {
                // Use SIGUSR1 to log metrics.  
            }
            _ => {
            	// Omit others
            }
        }
    }
}

fn setup_log() {
	/*if rlog::check_file_size_exceeded_max(&filename) {
		let backupfilename = filename.push_str(".bak");
		fs::rename(&filename, backupfilename);
	}*/

	rlog::setup_logging(0, "/rbctrl/apiserver/log/rust.log".to_string()).expect("Failed to initialize logging.");
}

fn main() {
	setup_log();

	info!("starting websocket server...");
	let (websocket_tx, websocket_rx) = mpsc::channel::<Vec<u8>>();
	let wss = ws::new_websocket_server("0.0.0.0:9050");
	thread::spawn(move || {
		wss.run(websocket_rx);
	});

	info!("starting shm server...");
	let shmserver = shm::new_shm_server();
	shmserver.init().run(websocket_tx.clone());
    handle_signals();
}