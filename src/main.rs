mod api;

fn main() {
    let apiserver = api::NewApiServer();
    apiserver.configServerHandler();
    apiserver.run();
}