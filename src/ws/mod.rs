use websocket::async::Server;
use websocket::message::{Message, OwnedMessage};
use websocket::server::InvalidConnection;

use futures::{Future, Sink, Stream};
use tokio_core::reactor::{Core, Handle};

pub struct WsServer {
	server: Server::
}

pub fn new_websocket_server() {

}