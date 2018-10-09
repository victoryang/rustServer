use rocket;

mod apiv1;

pub struct apiServer {
	r:	rocket::Rocket
}

impl apiServer {
	pub fn run(&mut self) {
		self.r.launch();
	}

	pub fn configServerHandler(mut self) -> Self {
		apiv1::registerAPIv1(mut self.r);
		self
	}
}

pub fn NewApiServer() -> apiServer {
	apiServer {
		r: rocket::ignite(),
	}
}
