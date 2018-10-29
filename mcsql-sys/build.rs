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
    //println!("cargo:static=1");

    match decompress("build/include/include.tar.gz", format!("{}", root.display()).as_str()) {
    	Ok(()) => {
    		fs::copy("build/include/config.h", root.join("config.h")).unwrap();
    		fs::copy("mcsql/define.h", include.join("define.h")).unwrap();
		    fs::copy("mcsql/mcmanager.h", include.join("mcmanager.h")).unwrap();
		    fs::copy("mcsql/mcquery.h", include.join("mcquery.h")).unwrap();
		    fs::copy("mcsql/mcsql.h", include.join("mcsql.h")).unwrap();
		    fs::copy("mcsql/mcsqlmapper.h", include.join("mcsqlmapper.h")).unwrap();
    	},
    	Err(_) => return,
    }

    fs::create_dir_all(&libdir).unwrap();
    fs::copy("build/lib/libz.so.1.2.8", libdir.join("libz.so.1.2.8")).unwrap();
    fs::copy("build/lib/libsqlitedb.so", libdir.join("libsqlitedb.so")).unwrap();

    Command::new("make").args(&["-C", "mcsql/"]).status().unwrap();
    fs::copy("mcsql/libmcsql.so", libdir.join("libmcsql.so")).unwrap();

    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=sqlitedb");
    println!("cargo:rustc-link-lib=dylib=mcsql");
    println!("cargo:rustc-link-search=native={}", libdir.display());
}