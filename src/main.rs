#![feature(custom_attribute)]
#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate text_template

#[macro_use]
extern crate log;
extern crate fern;

mod api;
mod middlewares;

fn main() {
    let apiserver = api::new_api_server();
    apiserver.config_server_handler().run();
}
