#![feature(custom_attribute)]
#![feature(plugin, decl_macro, proc_macro_non_items)]

mod api;

fn main() {
    let apiserver = api::NewApiServer();
    apiserver.configServerHandler();
    apiserver.run();
}