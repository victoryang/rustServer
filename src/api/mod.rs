use rocket;

mod apiv1;

pub struct apiServer {
	r:	rocket::Rocket
}

impl apiServer {
	pub fn run(&self) {
		self.r.launch();
	}

	pub fn configServerHandler(&self) {
		apiv1::registerAPIv1(&self.r);
	}
}

pub fn NewApiServer() -> apiServer {
	apiServer {
		r: rocket::ignite(),
	}
}
