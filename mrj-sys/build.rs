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
	let root = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let include = root.join("include");
    let libdir = root.join("lib");

    println!("cargo:root={}", root.display());
    println!("cargo:include={}", include.display());
    println!("cargo:libdir={}", libdir.display());
    println!("cargo:static=1");

    match decompress("build/include/include.tar.gz", format!("{}", root.display()).as_str()) {
    	Ok(()) => {
    		fs::copy("build/include/config.h", root.join("config.h")).unwrap();
    		fs::copy("mrj/mrj.h", include.join("mrj.h")).unwrap();
		    fs::copy("mrj/mrjresource.h", include.join("mrjresource.h")).unwrap();
            fs::copy("mrj/mrjsysvar.h", include.join("mrjsysvar.h")).unwrap();
		    fs::copy("mrj/mrjlocvar.h", include.join("mrjlocvar.h")).unwrap();
		    fs::copy("mrj/mrjplc.h", include.join("mrjplc.h")).unwrap();
		    fs::copy("mrj/mrjnv.h", include.join("mrjnv.h")).unwrap();
    	},
    	Err(_) => return,
    }

    fs::create_dir_all(&libdir).unwrap();
    fs::copy("build/lib/librobresource.so", libdir.join("librobresource.so")).unwrap();

    let env = format!("INCLUDE={}", include.display()); 
    Command::new("make").args(&[env.as_str(), "-C", "mrj/"]).status().unwrap();
    fs::copy("mrj/libmrj.a", libdir.join("libmrj.a")).unwrap();

    println!("cargo:rustc-link-lib=robresource");
    println!("cargo:rustc-link-lib=static=mrj");
    println!("cargo:rustc-link-search=native={}", libdir.display());
}
