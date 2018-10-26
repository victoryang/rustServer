extern crate flate2;
extern crate tar;

use std::process::Command;
use std::path::{PathBuf};
use std::env;
use std::fs;

use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;

fn decompress(src: &str, des: &str) -> Result<(), std::io::Error> {
	let tar_gz = File::open(src)?;
	let tar = GzDecoder::new(tar_gz);
	let mut archive = Archive::new(tar);
	archive.unpack(des)?;

	Ok(())
}

fn main() {
	let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let include = dst.join("include");
    let libdir = dst.join("lib");

    println!("cargo:root={}", dst.display());
    println!("cargo:include={}", include.display());
    println!("cargo:libdir={}", libdir.display());
    println!("cargo:static=1");

    match decompress("build/include/include.tar.gz", format!("{}", dst.display()).as_str()) {
    	Ok(()) => {
    		fs::copy("build/include/config.h", dst.join("config.h")).unwrap();
    		fs::copy("mrj/mrj.h", include.join("mrj.h")).unwrap();
		    fs::copy("mrj/mcresource.h", include.join("mcresource.h")).unwrap();
		    fs::copy("mrj/mcvars.h", include.join("mcvars.h")).unwrap();
		    fs::copy("mrj/mcplc.h", include.join("mcplc.h")).unwrap();
		    fs::copy("mrj/mcnv.h", include.join("mcnv.h")).unwrap();
    	},
    	Err(_) => return,
    }

    fs::create_dir_all(&libdir).unwrap();
    fs::copy("build/lib/libz.so.1.2.8", libdir.join("libz.so.1.2.8")).unwrap();
    fs::copy("build/lib/libshare.a", libdir.join("libshare.a")).unwrap();

    Command::new("make").args(&["-C", "mrj/"]).status().unwrap();
    fs::copy("mrj/libmrj.a", libdir.join("libmrj.a")).unwrap();

    println!("cargo:rustc-link-lib=static=share");
    println!("cargo:rustc-link-lib=static=mrj");
    println!("cargo:rustc-link-search=native={}", libdir.display());
}