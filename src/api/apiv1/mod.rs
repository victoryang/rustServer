//use rocket::routes;

#[get("/")]
fn hello() -> String {
	format!("hello world!")
}

pub fn registerAPIv1(r: rocket::Rocket) -> rocket::Rocket{
	r.mount("/", routes![hello]);
	r
}
