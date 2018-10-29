use std::sync::mpsc;
use std::thread;
use timer;
use chrono::Duration;
use mrj_sys;

mod shared;
mod nv;

static DURATION: i64 = 100;

pub struct ShmServer {
}

impl ShmServer {
	pub fn init(self) -> Self {
		mrj_sys::init_worker_resource();
		self
	}

	pub fn run(&self, websocket_tx: mpsc::Sender<Vec<u8>>) {
		thread::spawn(move || {
			let (tx, rx) = mpsc::channel::<Vec<u8>>();
			let timer = timer::Timer::new();
			timer.schedule_repeating(Duration::milliseconds(DURATION), move || {
				let tx_shared = tx.clone();
				shared::get_shared(tx_shared);

				let tx_plc = tx.clone();
				shared::get_plc(tx_plc);

				let tx_nv = tx.clone();
				nv::get_nv(tx_nv);
			});

			let mut iter = rx.iter();
			loop {
				match iter.next() {
					Some(m) => {websocket_tx.send(m).unwrap();},
					None => break,
				}
			}
		});
	}
}

pub fn new_shm_server() -> ShmServer {
	ShmServer {
	}
}