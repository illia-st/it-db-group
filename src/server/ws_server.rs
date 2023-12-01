use std::net::TcpListener;
use std::rc::Rc;
use simple_websockets::{Event, EventHub, Message};
use crate::db_manager::DatabaseManager;
use super::command_dispatcher::CommandDispatcher;
use super::ws_context::WsContext;

#[derive(Debug, Default)]
pub struct WsServer {
    context: WsContext,
    event_hub: Option<EventHub>,
    dispatcher: CommandDispatcher,
}

// impl Default for WsServer {
//     fn default() -> Self {
//         Self {
//             context: WsContext::default(),
//             event_hub: None,
//             dispatcher: CommandDispatcher::default(),
//         }
//     }
// }

impl WsServer {
    pub fn bind(mut self, endpoint: String) -> Self {
        let listener = TcpListener::bind(endpoint.as_str())
            .unwrap_or_else(|_| panic!("failed to bind web socket on address {}", endpoint.as_str()));
        self.event_hub = Some(
            simple_websockets::launch_from_listener(listener)
                .unwrap_or_else(|_| panic!("failed to listen on address {}", endpoint.as_str()))
        );
        self
    }
    pub fn get_context(&self) -> WsContext {
        self.context.clone()
    }
    pub fn poll(&self, events_count: i32) {
        let mut counter = 0;
        while counter != events_count {
            match self.event_hub.as_ref().unwrap().poll_event() {
                Event::Connect(connection_id, responder) => {
                    log::debug!("a new connection with id #{}", connection_id);
                    self.context.add_connection(connection_id, responder);
                    log::debug!("amount of connections {}", self.context.get_size());
                }
                Event::Disconnect(connection_id) => {
                    log::debug!("a connection with id #{} is disconnected.", connection_id);
                    self.context.remove_connection(connection_id);
                    log::debug!("amount of connections {}", self.context.get_size());
                }
                Event::Message(connection_id, message) => {
                    let message = match message {
                        Message::Binary(data) => data,
                        _ => {
                            log::error!("only binary form of messages is supported");
                            continue;
                        }
                    };
                    let responder = if let Some(responder) = self.context.get_connection_by_id(connection_id) {
                        responder
                    } else {
                        continue;
                    };
                    let response = self.dispatcher.dispatch(message);
                    responder.send(Message::Binary(response));
                }
            }
            counter += 1;
        }
    }
    pub fn into_inner(self) -> Rc<Self> {
        Rc::new(self)
    }
}