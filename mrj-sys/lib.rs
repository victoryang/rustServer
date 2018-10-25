#![feature(libc)]
extern crate libc;

extern {fn init_nv_ram();}
extern {fn mrj_init();}

use libc::{c_int};

pub fn init_worker_resource() {
	unsafe {
		init_nv_ram();
		mrj_init();
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
