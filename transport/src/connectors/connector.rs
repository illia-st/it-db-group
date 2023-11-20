use std::rc::Rc;
use std::sync::Arc;

use super::core::{
    Receiver,
    Sender,
    Handler,
    Socket,
};

pub struct ReplyConnector<HANDLER: Handler> {
    endpoint: String,
    handler: Rc<HANDLER>,
    socket: zmq::Socket,
    #[allow(dead_code)]
    context: Arc<zmq::Context>,
}

impl<HANDLER: Handler> Receiver for ReplyConnector<HANDLER> {
    fn recv(&self) -> Vec<u8> {
        self.socket.recv_bytes(0)
            .expect("connector failed receiving data")
    }
}

impl<HANDLER: Handler> Sender for ReplyConnector<HANDLER> {
    fn send(&self, data: &[u8]) {
        self.socket.send(data, 0)
            .expect("client failed sending data");
    }
}

impl<HANDLER: Handler> Socket for ReplyConnector<HANDLER> {

    fn get_socket(&self) -> &zmq::Socket {
        &self.socket
    }

    fn handle(&self, receiver: &dyn Receiver, sender: &dyn Sender) {
        self.handler.handle(receiver, sender);
    }

    fn get_receiver(&self) -> &dyn Receiver {
        self
    }

    fn get_sender(&self) -> &dyn Sender {
        self
    }
}

impl<HANDLER: Handler> ReplyConnector<HANDLER> {
    pub fn new(endpoint: String, handler: Rc<HANDLER>, socket: zmq::Socket, context: Arc<zmq::Context>) -> Self {
        Self {
            endpoint,
            handler,
            socket,
            context,
        }
    }
    pub fn bind(self) -> Self {
        self.socket.bind(&self.endpoint)
            .expect("couldn't bind a connector");
        self
    }
    pub fn connect(self) -> Self {
        self.socket.connect(&self.endpoint)
            .expect("couldn't establish a connection");
        self
    }
    pub fn into_inner(self) -> Rc<Self> {
        Rc::from(self)
    }
}