pub fn get_all_arc() -> Vec<u8> {
	match query() {
		Ok(m) => m,
		Err(e) => {
			warn!("query fails");
		},
	}
}