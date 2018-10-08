#![feature(custom_attribute)]
#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod api;

fn main() {
    let apiserver = api::NewApiServer();
    apiserver.configServerHandler();
    apiserver.run();
}
