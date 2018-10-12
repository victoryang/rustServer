use rocket;

mod apiv1;

pub struct Apiserver {
	r:	rocket::Rocket
}

impl Apiserver {
	pub fn run(self) {
		self.r.launch();
	}

	pub fn config_server_handler(mut self) -> Self {
		self.r = apiv1::register_apiv1(self.r);
		self
	}
}

fn get_middlewares(mut r: rocket::Rocket) -> rocket::Rocket {
	r.attach(super::middlewares::accesslog.new())
}

pub fn new_api_server() -> Apiserver {
	let r = rocket::ignite();
	Apiserver {
		r: get_middlewares(r),
	}
}
