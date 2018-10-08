extern crate rocket;

#[get("/")]
fn hello() -> String {
	format!("hello world!")
}

pub fn registerAPIv1(&r: rocket::Rocket) {
	r.mount("/", routes![hello]);
}