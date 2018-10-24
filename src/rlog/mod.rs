use fern;
use std::fs;
use std::io;
use log;
use chrono;

pub fn setup_logging(verbosity: u64, filename: String) -> Result<(), fern::InitError> {
	let mut gobal_config = fern::Dispatch::new();

	gobal_config = match verbosity {
		0 => {
			gobal_config
				.level(log::LevelFilter::Info)
		}
		1 => gobal_config.level(log::LevelFilter::Debug),
		_2_or_more => gobal_config.level(log::LevelFilter::Trace),
	};

	let file_config = fern::Dispatch::new()
		.format(|out, message, record| {
	        out.finish(format_args!(
	            "time=\"{}\" [{}][{}] msg=\"{}\"",
	            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
	            record.target(),
	            record.level(),
	            message
	        ))
	    })
	    .level(log::LevelFilter::Info)
	    .chain(fern::log_file(filename)?);

	/*let stdout_config = fern::Dispatch::new()
		.format(|out, message, record| {
			if record.level() > log::LevelFilter::Info {
				out.finish(format_args!(
                    "---\nDEBUG: {}: {}\n---",
                    chrono::Local::now().format("%H:%M:%S"),
                    message
                ))
			}
		})
		.chain(io::stdout());*/

	gobal_config.chain(file_config)./*chain(stdout_config).*/apply()?;

	Ok(())
}

pub fn check_file_size_exceeded_max(filename: &String) -> bool {
	match fs::metadata(filename) {
		Ok(metadata) => {
			if metadata.len() > 32<<20 {
				true
			} else {
				false
			}
		}
		Err(_) => false
	}
}