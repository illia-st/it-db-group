use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use simple_websockets::{Responder};


#[derive(Default, Debug, Clone)]
pub struct WsContext {
    ws_connections: Arc<RwLock<HashMap<u64, Responder>>>,
}

impl WsContext {
    pub fn set_context(&mut self, context: WsContext) {
        self.ws_connections = context.get_raw();
    }

    pub fn get_connections(&self) -> Option<HashMap<u64, Responder>> {
        match self.ws_connections.read() {
            Ok(ws_connections) => {
                Some(ws_connections.deref().to_owned())
            },
            Err(err) => {
                log::error!("couldn't read context: {}", err);
                None
            },
        }
    }

    fn get_raw(self) -> Arc<RwLock<HashMap<u64, Responder>>> {
        self.ws_connections
    }

    pub fn get_size(&self) -> usize {
        match self.ws_connections.read() {
            Ok(ws_connections) => {
                ws_connections.len()
            }
            Err(err) => {
                log::error!("couldn't read context: {}", err);
                0
            }
        }
    }

    pub fn get_connection_by_id(&self, connection_id: u64) -> Option<Responder> {
        match self.ws_connections.read() {
            Ok(ws_connections) => {
                if let Some(responder) = ws_connections.get(&connection_id) {
                    return Some(responder.to_owned());
                }
                None
            }
            Err(err) => {
                log::error!("couldn't read context: {}", err);
                None
            }
        }
    }

    pub fn add_connection(&self, connection_id: u64, responder: Responder) {
        match self.ws_connections.write() {
            Ok(mut ws_connections) => {
                if ws_connections.insert(connection_id, responder).is_some() {
                    log::debug!("updated value of connection {}", connection_id);
                } else {
                    log::debug!("inserted a connection with id {}", connection_id);
                }
            }
            Err(err) => {
                log::error!("couldn't write to context: {}", err);
            }
        }
    }

    pub fn remove_connection(&self, connection_id: u64) -> bool {
        match self.ws_connections.write() {
            Ok(mut ws_connections) => {
                if ws_connections.remove(&connection_id).is_some() {
                    log::debug!("removed connection with id {}", connection_id);
                    return true;
                }
                log::debug!("there is no connection with id {} to be removed", connection_id);
                false
            }
            Err(err) => {
                log::error!("couldn't write to context: {}", err);
                false
            }
        }
    }
}