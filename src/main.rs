/* To enable macro select!*/
#![feature(mpsc_select)]

extern crate websocket;

extern crate jsonrpc_tcp_server;
#[macro_use] extern crate serde_derive;
extern crate serde;

extern crate timer;
extern crate chrono;
extern crate fern;

#[cfg(unix)]
extern crate signal;
extern crate nix;

#[macro_use]
extern crate log;

extern crate mrj_sys;
extern crate mcsql_sys;

use std::sync::mpsc;
use std::thread;
use std::fs;

use signal::trap::Trap;
use nix::sys::signal::{SIGUSR1, SIGHUP, SIGINT, SIGTERM};

mod shm;
mod ws;
mod rlog;
mod rpc;
//mod sqlitedb;

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

fn setup_log(src: String) {
	if rlog::check_file_size_exceeded_max(&src) {
		let mut des = src.clone();
        des.push_str(".bak")
		match fs::rename(src.as_str(), des.as_str()) {
            Ok(()) => info!("log backuped"),
            Err(_) => info!("log fails to be backuped"),
        };
	};

	rlog::setup_logging(0, src).expect("Failed to initialize logging.");
}

fn main() {
	setup_log("/rbctrl/apiserver/log/rust.log".to_string());

	info!("starting websocket server...");
	let (websocket_tx, websocket_rx) = mpsc::channel::<Vec<u8>>();
	let wss = ws::new_websocket_server("0.0.0.0:9050");
	thread::spawn(move || {
		wss.run(websocket_rx);
	});

	info!("starting shm server...");
	let shmserver = shm::new_shm_server();
	shmserver.init().run(websocket_tx.clone());

    info!("starting rpc server...");
    let rpcserver = rpc::new_rpc_server();
    rpcserver.run(websocket_tx.clone());

    handle_signals();
}