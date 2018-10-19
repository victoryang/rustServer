use std::sync::mpsc;

use ::ws;

mod worker;
mod shared;
mod nv;

pub struct ShmServer {
	wss:	ws::WsServer,
	hit:	(mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>),
}

pub fn new_shm_server() {
	worker::init_worker_resource()
		.init_watch_funcs();
}