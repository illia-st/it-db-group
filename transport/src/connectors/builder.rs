use std::rc::Rc;
use std::sync::Arc;
use super::core::Handler;
use super::connector::Connector;

pub struct ConnectorBuilder<HANDLER: Handler> {
    context: Option<Arc<zmq::Context>>,
    endpoint: Option<String>,
    handler: Option<Rc<HANDLER>>,
}

impl<HANDLER: Handler> ConnectorBuilder<HANDLER> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ConnectorBuilder {
            context: None,
            endpoint: None,
            handler: None,
        }
    }
    pub fn with_handler(mut self, handler: Rc<HANDLER>) -> Self {
        self.handler = Some(handler);
        self
    }
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    pub fn with_context(mut self, context: Arc<zmq::Context>) -> Self {
        self.context = Some(context);
        self
    }

    pub fn build_requester(self) -> Connector<HANDLER> {
        self.build(zmq::REQ)
    }

    pub fn build_replyer(self) -> Connector<HANDLER> {
        self.build(zmq::REP)
    }

    fn build(self, socket_type: zmq::SocketType) -> Connector<HANDLER> {
        let context = self.context.unwrap();
        let socket = context.socket(socket_type).unwrap();
        Connector::new(
            self.endpoint.as_ref().unwrap().to_string(),
            self.handler.as_ref().unwrap().clone(),
            socket,
            context,
        )
    }
}
