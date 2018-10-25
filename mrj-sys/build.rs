use std::process::Command;
use std::path::{PathBuf};
use std::env;
use std::fs;

fn main() {
	Command::new("make").args(&["-C", "mrj/"])
						.status().unwrap();

	let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let include = dst.join("include");
    let build = dst.join("build");

    println!("cargo:root={}", dst.display());
    println!("cargo:include={}", include.display());
    println!("cargo:static=1");
    fs::create_dir_all(include.join("mrj")).unwrap();
    fs::copy("mrj/mrj.h", include.join("mrj/mrj.h")).unwrap();
    fs::copy("mrj/mcresource.h", include.join("mrj/mcresource.h")).unwrap();
    fs::copy("mrj/mcvars.h", include.join("mrj/mcvars.h")).unwrap();
    fs::copy("mrj/mcplc.h", include.join("mrj/mcplc.h")).unwrap();
    fs::copy("mrj/mcnv.h", include.join("mrj/mcnv.h")).unwrap();

	println!("cargo:rustc-link-lib=static=mrj");
	println!("cargo:rustc-link-search=native==mrj");
}