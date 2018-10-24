use std::sync::mpsc;
use std::thread;
use timer;
use chrono::Duration;

mod shared;
mod nv;

static DURATION: i64 = 100;

pub struct ShmServer {
	websocket_tx:	mpsc::Sender<Vec<u8>>,
}

fn init_worker_resource() {
	println!("init");
}

impl ShmServer {
	pub fn init(self) -> Self {
		init_worker_resource();
		self
	}

	pub fn run(&self) {
		let websocket_tx = self.websocket_tx.clone();

		thread::spawn(move || {
			let (tx, rx) = mpsc::channel::<Vec<u8>>();
			thread::spawn(move || {
				let timer = timer::Timer::new();
				timer.schedule_repeating(Duration::milliseconds(DURATION), move || {
					shared::get_shared(tx.clone());
					shared::get_plc(tx.clone());
					nv::get_nv(tx.clone());
				})
			});

			for m in rx.try_recv() {
				websocket_tx.send(m).unwrap();
			}
		});	
	}
}

pub fn new_shm_server(websocket_tx: mpsc::Sender<Vec<u8>>) -> ShmServer {
	ShmServer {
		websocket_tx,
	}
}