use std::fs;
use rocket::{Request, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::handler::Outcome;
use rocket::request::{self, FromRequest};
use text_template;
use std::collections::HashMap;
use std::time::SystemTime;
use log;
use fern;

static LoggerDefaultName: &'static str = "api-server ";

static LoggerDefaultFormat: text_template::Template<'_> = text_template::Template::from("${StartTime} | ${Status} | \t ${Duration} | ${Hostname} | ${Method} ${Path} \n");

#[derive(Copy, Clone)]
struct TimerStart(Option<SystemTime>);

#[derive(Copy, Clone)]
pub struct StartTime(pub SystemTime);

impl<'a, 'r> FromRequest<'a, 'r> for StartTime {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<StartTime, ()> {
		match *request.local_cache(|| TimerStart(None)) {
			TimerStart(Some(time)) => Outcome::Success(StartTime(time)),
			TimerStart(None) => Outcome::Failure((Status::InternalServerError, ())),
		}
	}
}

pub struct Logger {
	Filename: String,
}

impl Fairing for Logger {
	// This is a middleware for access log
	fn info(&self) -> Info {
		Info {
			name: "Access log middleware",
			kind: Kind::Request | Kind::Response,
		}
	}

	fn on_request(&self, request: &mut Request, _: &Data) {
		request.local_cache(|| TimerStart(Some(SystemTime::now())));
	}

	fn on_response(&self, request: &Request, response: &mut Response) {
		let start_time = request.local_cache(|| TimerStart(None));
		let mut ms: u64 = 0;
		if let Some(Ok(duration)) = start_time.0.map(|st| st.elapsed()) {
            ms = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
        }

		let mut values = HashMap::new();
		values.insert("StartTime", start_time);
		values.insert("Status", response.status());
		values.insert("Duration", ms);
		values.insert("Hostname", "192.168.1.253:9000");
		values.insert("Method", request.method().as_str());
		values.insert("Path", request.uri().path());

		let text = LoggerDefaultFormat.fill_in(&values);

		info!("{}",text.to_string());
	}
}

pub fn new(filename: String) -> Logger {
	if check_file_size_exceeded_max(&filename) {
		let backupfilename = filename.push_str(".bak");
		fs::rename(&filename, backupfilename);
	}

	setup_logger(&filename);

	Logger {
		Filename: filename,
	}
}

fn setup_logger(filename: &String) -> Result<(), fern::InitError> {
	fern::Dispatch::new()
		.format(|out, message, record| {
	        out.finish(format_args!(
	            "{} {}",
	            LoggerDefaultName,
	            message
	        ))
	    })
	    .level(log::LevelFilter::Info)
	    .chain(fern::log_file(filename)?)
	    .apply()?;
	Ok(())
}

fn check_file_size_exceeded_max(filename: &String) -> bool {
	match fs::metadata(filename) {
		Ok(metadata) => {
			if metadata.len() > 32<<20 {
				true
			} else {
				false
			}
		}
		Err() => false
	}
}